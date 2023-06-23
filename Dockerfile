FROM ubuntu:22.04

RUN sed -i 's/archive.ubuntu.com/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

WORKDIR /app
COPY target/release  .

EXPOSE 50011

CMD ["./frontend_base_service"]