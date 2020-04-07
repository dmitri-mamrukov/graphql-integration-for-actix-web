FROM rust:1 as builder

COPY . .

RUN cargo build --release

FROM rust:1-slim-stretch

COPY --from=builder /target/release/crud_server_sample .

RUN ls -la /crud_server_sample

EXPOSE 8000

ENTRYPOINT ["/crud_server_sample"]
