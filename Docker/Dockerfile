FROM rust:1.76 as development
WORKDIR /usr/src/prj-choco
RUN apt update && apt install -y \
    build-essential \
    cmake \
    git \
    libssl-dev \
    pkg-config \
    vim \
    wget \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*
CMD ["bash"]

# ビルド専用ステージ
FROM rust:1.76 as builder
WORKDIR /usr/src/prj-choco
COPY . .
RUN cargo test && cargo build --release

# 実行専用ステージ
FROM debian:buster-slim
WORKDIR /usr/local/bin
# ビルドステージからコンパイル済みバイナリをコピー
COPY --from=builder /usr/src/prj-choco/target/release/prj-choco .
# 実行時にバイナリを起動
CMD ["./prj-choco"]
