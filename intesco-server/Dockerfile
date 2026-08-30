FROM rust:1.88-bullseye AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev ca-certificates
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
RUN cargo build --release

FROM debian:11-slim
COPY --from=builder /app/target/release/intesco-server /usr/local/bin/webserver
RUN chmod +x /usr/local/bin/webserver
RUN apt-get update && apt-get install -y ca-certificates
ENTRYPOINT ["/usr/local/bin/webserver"]

