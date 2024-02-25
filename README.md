# prj-choco  
これはRust言語、テスト、CIの練習用リポジトリです。
ChatGPTとGithub Copilotとやりとりしながら作成しています。

# Github Actions Status
[![Rust CI Pipeline](https://github.com/bowplusplus/prj-choco/actions/workflows/workflows.yml/badge.svg)](https://github.com/bowplusplus/prj-choco/actions/workflows/workflows.yml)

# 実行手順
```
docker image build -t bowplusplus/my-rust:v0.0.1 -f ./Docker/Dockerfile .
```

```
docker container run --name my-rust --rm bowplusplus/my-rust:v0.0.1
```