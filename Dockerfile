FROM rust:1.92.0-trixie as builder

RUN apt-get update && apt-get install -y \
    curl unzip clang pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

ENV NVM_DIR /root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash \
    && . "$NVM_DIR/nvm.sh" \
    && nvm install 24 \
    && nvm use 24 \
    && nvm alias default 24

ENV PATH $NVM_DIR/versions/node/v24.14.1/bin:$PATH

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY package.json bun.lockb* ./
RUN bun install

COPY . .

RUN CARGO_BUILD_JOBS=2 cargo leptos build --release -vv

FROM debian:trixie-slim as runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates libgcc-s1 \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /app/chara
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

RUN ls -R

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/server"]
