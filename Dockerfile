# Build Stage
FROM rust:latest as builder
WORKDIR /app

COPY . .
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

FROM alpine:latest

# RUN apk update -y \
#     && apk install -y ca-certificates \
#     && apk upgrade -y

EXPOSE 3000

COPY --from=builder /app/target/release/kayo-api /app/kayo-api

CMD ["/app/kayo-api"]
