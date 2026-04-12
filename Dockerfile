FROM rust:latest
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    binaryen \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked cargo-leptos
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_OUTPUT_NAME=your_app_name
EXPOSE 3000
CMD ["cargo", "leptos", "serve", "--release"]
