# Stage 1: Build Rust binary
FROM rust:1.82-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Minimal Debian runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/trace_backend /app/trace_backend
EXPOSE 8080
ENV PORT=8080
CMD ["/app/trace_backend"]
