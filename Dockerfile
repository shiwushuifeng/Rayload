# syntax=docker/dockerfile:1

# Headless rayload-cli image. The service never runs as root: the final stage
# creates an unprivileged `rayload` user and drops to it before startup.

FROM rust:1-bookworm AS builder
WORKDIR /build

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libwebkit2gtk-4.1-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src-tauri ./src-tauri
COPY dist ./dist

RUN cargo build --release --manifest-path src-tauri/Cargo.toml --features cli --bin ray

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends aria2 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --home-dir /data --shell /usr/sbin/nologin rayload \
    && mkdir -p /downloads /data \
    && chown -R rayload:rayload /downloads /data

COPY --from=builder /build/src-tauri/target/release/ray /usr/local/bin/ray
RUN ln -s /usr/local/bin/ray /usr/local/bin/rayload

ENV RAYLOAD_ARIA2_BIN=/usr/bin/aria2 \
    RAYLOAD_DOWNLOAD_DIR=/downloads \
    RAYLOAD_DATA_DIR=/data

USER rayload
EXPOSE 6767
VOLUME ["/downloads", "/data"]

ENTRYPOINT ["ray"]
CMD ["start"]
