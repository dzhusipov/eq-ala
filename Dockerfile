FROM rust:latest as build
WORKDIR /app
COPY . .

# add target musl for x86_64
RUN rustup target add x86_64-unknown-linux-musl
# add target musl for arm
# RUN rustup target add aarch64-unknown-linux-musl 
# RUN rustup toolchain install stable-aarch64-unknown-linux-musl

# add apt musl packages
RUN apt-get update && apt-get install -y musl-tools

# RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

# FROM debian:bullseye-slim
# FROM alpine:latest
FROM scratch
WORKDIR /app
# COPY --from=build /app/target/release/zhus_sip /app/zhus_sip
# COPY --from=build /app/target/x86_64-unknown-linux-musl/release/zhus_sip /app/zhus_sip
COPY --from=build /app/target/aarch64-unknown-linux-musl/release/eq-ala /app/eq-ala
COPY --from=build /app/config /app/config
COPY --from=build /app/.env_docker /app/.env

EXPOSE 5060
EXPOSE 8080
CMD ["/app/eq-ala"]