FROM rust:latest
WORKDIR /app
COPY . .
RUN ls
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-leptos
RUN cargo leptos build --release
EXPOSE 3000
CMD ["cargo", "leptos", "serve", "--release"]
