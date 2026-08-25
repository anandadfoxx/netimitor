FROM docker.io/rust:1.98-trixie AS builder

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    perl \
    pkg-config \
    libclang-dev \
    musl-tools \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian13 AS runtime

COPY --from=builder --chown=1000:1000 /app/target/release/netimitor /netimitor

USER 1000:1000

ENTRYPOINT ["/netimitor"]
