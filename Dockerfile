# Use the standard Rust bookworm image for x86_64
FROM rust:bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl unzip clang pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"

# Install cargo-binstall and cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Install JS dependencies
COPY package.json bun.lock* pnpm-lock.yaml* ./
RUN if [ -f bun.lock ]; then bun install; \
    elif [ -f pnpm-lock.yaml ]; then npm install -g pnpm && pnpm install; \
    else npm install; fi

# Copy source code
COPY . .

# Build the application for x86_64
RUN cargo leptos build --release

# Normalize artifacts path
RUN mkdir -p /app/out && \
    cp target/release/server /app/out/chara && \
    cp -r target/site /app/out/site

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy artifacts from the builder's out directory
COPY --from=builder /app/out/ /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/chara"]
