# prj-choco  
これはRust言語、テスト、CIの練習用リポジトリです。
ChatGPTとGithub Copilotとやりとりしながら作成しています。

# Github Actions Status
[![Rust CI Pipeline](https://github.com/bowplusplus/prj-choco/actions/workflows/workflows.yml/badge.svg)](https://github.com/bowplusplus/prj-choco/actions/workflows/workflows.yml)

# 開発環境コンテナ実行手順
```
docker image build --target development -t bowplusplus/my-rust:v0.0.1.dev -f ./Docker/Dockerfile .
```

```
docker container run -itd --name my-rust.dev -v ${PWD}/src:/usr/src/prj-choco/src -v ${PWD}/Cargo.toml:/usr/src/prj-choco/Cargo.toml bowplusplus/my-rust:v0.0.1.dev
```