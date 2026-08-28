FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY static ./static
RUN cargo build --release

# Production image — only the compiled binary is copied, so secrets mounted
# into /app/secrets at runtime (see docker-compose.demo.yml) never pass
# through the build stage. debian:bookworm-slim matches the builder's glibc
# (rust:1-bookworm) so the binary runs without a static-linking step.
FROM debian:bookworm-slim AS runner

# curl is only for the docker-compose healthcheck (debian-slim ships neither
# curl nor wget by default).
RUN apt-get update && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ai-sandbox-rust-demo ./server

EXPOSE 3000
CMD ["./server"]
