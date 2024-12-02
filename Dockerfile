FROM rust:1.72 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-data-ingest /app/
COPY prometheus.yml /etc/prometheus/prometheus.yml
ENTRYPOINT ["/app/rust-data-ingest"]
