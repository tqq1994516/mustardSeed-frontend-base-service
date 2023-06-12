FROM rust:alpine AS builder

ENV GO111MODULE=on \
    GOPROXY=https://goproxy.cn,direct \
    CGB_ENABLED=0 \
    GOOS=linux \
    GOARCH=amd64

WORKDIR /build
COPY . .

RUN cargo build

FROM scratch
# 从builder镜像中把/dist/app 拷贝到当前目录
COPY --from=builder /build/app /

EXPOSE 50011

CMD ["/app"]

