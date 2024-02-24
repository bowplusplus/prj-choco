# ビルド専用ステージ
FROM rust:1.56 as builder
WORKDIR /usr/src/prj-choco
COPY . .
RUN cargo build --release

# 実行専用ステージ
FROM debian:buster-slim
WORKDIR /usr/local/bin
# ビルドステージからコンパイル済みバイナリをコピー
COPY --from=builder /usr/src/prj-choco/target/release/prj-choco .
# 実行時にバイナリを起動
CMD ["./prj-choco"]
