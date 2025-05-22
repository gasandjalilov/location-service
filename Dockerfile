FROM rust:alpine3.21 as builder
RUN apk add musl-dev
WORKDIR app
COPY . .
RUN cargo build --release

FROM alpine:latest as runtime
COPY --from=builder /app/target/release/location-service /
CMD ["./location-service"]