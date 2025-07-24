FROM rust:1.88.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/base-debian12
WORKDIR /app
COPY --from=builder /app/target/release/echo .
ENTRYPOINT [ "/app/echo" ]
