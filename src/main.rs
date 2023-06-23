use std::collections::HashMap;
use dapr::{
    appcallback::*,
    dapr::dapr::proto::runtime::v1::app_callback_server::{AppCallback, AppCallbackServer},
};
use prost::Message;
use nacos_sdk::api::config::{ConfigServiceBuilder, ConfigService};
use nacos_sdk::api::props::ClientProps;
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
        ).build().unwrap();
        let response: UniversalReply;
        match method as &str {
            "getI18n" => {
                let data = &r.data;
                if let Some(any) = data {
                    let data = &any.value;
                    let resp = I18nRequest::decode(&data[..]).unwrap();
                    let config_resp = config_service.get_config(format!("i18n-{}", resp.lang), "frontend-base-service".to_string()).await.unwrap();
                    let lang_map: HashMap<String, String> = serde_yaml::from_str(&config_resp.content()).unwrap();
                    response = UniversalReply {
                        message: serde_json::to_string(&lang_map).unwrap(),
                    };
                } else {
                    return Ok(Response::new(InvokeResponse::default()))
                }
            },
            "getRoute" => {
                let config_resp = config_service.get_config("frontend-route".to_string(), "frontend-base-service".to_string()).await.unwrap();
                response = UniversalReply {
                    message: config_resp.content().to_string(),
                };
            },
            _ => return Ok(Response::new(InvokeResponse::default())),
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
        return Ok(Response::new(invoke_response))
    }
    
    async fn list_topic_subscriptions(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListTopicSubscriptionsResponse>, Status> {
        let list_subscriptions = ListTopicSubscriptionsResponse::default();
        Ok(Response::new(list_subscriptions))
    }

    /// Subscribes events from Pubsub.
    async fn on_topic_event(
        &self,
        _request: Request<TopicEventRequest>,
    ) -> Result<Response<TopicEventResponse>, Status> {
        Ok(Response::new(TopicEventResponse::default()))
    }

    /// Lists all input bindings subscribed by this app.
    async fn list_input_bindings(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListInputBindingsResponse>, Status> {
        Ok(Response::new(ListInputBindingsResponse::default()))
    }

    /// Listens events from the input bindings.
    async fn on_binding_event(
        &self,
        _request: Request<BindingEventRequest>,
    ) -> Result<Response<BindingEventResponse>, Status> {
        Ok(Response::new(BindingEventResponse::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = "[::]:50011".parse()?;
    let callback_service = AppCallbackService {};

    Server::builder()
        .add_service(AppCallbackServer::new(callback_service))
        .serve(server_address)
        .await?;

    Ok(())
}
