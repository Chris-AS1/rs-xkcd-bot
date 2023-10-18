# Create cargo-chef layer
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release 

# Run binary on Bookworm
FROM debian:bookworm-slim AS runtime
RUN apt update && apt upgrade
# Needed for teloxide
RUN apt install openssl ca-certificates -y
WORKDIR /app
RUN mkdir ./config
COPY --from=builder /app/target/release/oxotly_bot /usr/local/bin/app
COPY ./config/* ./config
ENTRYPOINT ["/usr/local/bin/app"]
