FROM rust:1.74.1-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

# setup yew
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY ./app /app
RUN cargo build
CMD trunk serve --address 0.0.0.0