use dapr::{
    appcallback::*,
    dapr::dapr::proto::runtime::v1::app_callback_server::{AppCallback, AppCallbackServer},
};
use prost::Message;
use tonic::{transport::Server, Request, Response, Status};

use frontend_base_service::{I18nRequest, UniversalReply};

pub mod frontend_base_service {
    tonic::include_proto!("frontend_base_service"); // The string specified here must match the proto package name
}

pub struct AppCallbackService {}

#[tonic::async_trait]
impl AppCallback for AppCallbackService {
    /// Invokes service method with InvokeRequest.
    async fn on_invoke(
        &self,
        request: Request<InvokeRequest>,
    ) -> Result<Response<InvokeResponse>, Status> {
        let r = request.into_inner();

        let method = &r.method;
        let config_service = ConfigServiceBuilder::new(
            ClientProps::new()
                .server_addr("nacos-cs:8848")
                .namespace("mustard-seed"),
        ).build()?;
        match method {
            'getI18n' => {
                let data = &r.data;
                if let Some(any) = data {
                    let data = &any.value;
                    let resp = I18nRequest::decode(&data[..]).unwrap();
                    let config_resp = &config_service.get_config(format!("i18n-{}", data.lang), "frontend-base-service".to_string());
                    match config_resp {
                        Ok(config_resp) => tracing::info!("get the config {}", config_resp),
                        Err(err) => tracing::error!("get the config {:?}", err),
                    }
                    let response = UniversalReply {
                        message: serde_json::to_string(serde_yaml::from_str(config_resp)?)?,
                    };
                };

            },
            'getRoute' => {
                let config_resp = &config_service.get_config("frontend-route".to_string(), "frontend-base-service".to_string());
                match config_resp {
                    Ok(config_resp) => tracing::info!("get the config {}", config_resp),
                    Err(err) => tracing::error!("get the config {:?}", err),
                }
                let response = UniversalReply {
                    message: config_resp,
                };
            },
            _ => return Ok(Response::new(InvokeResponse::default()));
        }
        let data = response.encode_to_vec();
        
        let data = prost_types::Any {
            type_url: "".to_string(),
            value: data,
        };

        let invoke_response = InvokeResponse {
            content_type: "application/json".to_string(),
            data: Some(data),
        };
        Ok(Response::new(invoke_response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = "[::]:50052".parse()?;
    let callback_service = AppCallbackService {};

    Server::builder()
        .add_service(AppCallbackServer::new(callback_service))
        .serve(server_address)
        .await?;

    Ok(())
}
