# Use --platform=$BUILDPLATFORM to ensure the builder runs on the native architecture of the runner (usually amd64)
# even when targeting arm64, which significantly speeds up the build via cross-compilation.
FROM --platform=$BUILDPLATFORM rust:1.81-bookworm AS builder

# Arguments for target platform
ARG TARGETPLATFORM
ARG BUILDPLATFORM

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

# Set architecture-specific build environment and build
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    apt-get update && apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu && \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc && \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc && \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ && \
    export PKG_CONFIG_ALLOW_CROSS=1 && \
    cargo leptos build --release --bin-target aarch64-unknown-linux-gnu; \
    else \
    cargo leptos build --release; \
    fi

# Normalize artifacts path
RUN mkdir -p /app/out && \
    if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        cp target/aarch64-unknown-linux-gnu/release/server /app/out/chara; \
    else \
        cp target/release/server /app/out/chara; \
    fi && \
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
