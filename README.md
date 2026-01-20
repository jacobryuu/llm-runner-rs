# LLM Runner RS

RustによるシンプルなLLM（Large Language Model）推論ランナーです。`llama.cpp`のRustバインディングを使用しており、GGUF形式のモデルをサポートしています。

## 必要条件

- Rust (Cargo)
- `curl` または `wget`

## 準備

### 1. モデルのダウンロード

このプロジェクトでは、GGUF形式のLLMモデルが必要です。付属のスクリプトを使用して、いくつかの推奨モデルを簡単にダウンロードできます。

まず、スクリプトに実行権限を与えます。
```bash
chmod +x download_model.sh
```

次に、モデル名を指定してスクリプトを実行します。

**Gemma 2 2B (推奨)**: バランスの良い性能と日本語能力
```bash
./download_model.sh gemma2b
```

**Qwen 2.5 3B**: 高い日本語性能
```bash
./download_model.sh qwen3b
```

**TinyLlama 1.1B**: 非常に軽量
```bash
./download_model.sh tinyllama
```

### 2. プロジェクトのビルド

```bash
cargo build --release
```

## 使い方

`cargo run` コマンドを使用して直接実行するか、ビルドしたバイナリを実行します。

### 基本的な使用法

```bash
cargo run --release -- --model-path <モデルへのパス> --prompt "<プロンプト>"
```

### 実行例

ダウンロードした`Gemma 2 2B`モデルを使って実行する例です。

**CPUのみで実行:**

```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --prompt "Rustプログラミング言語の利点は何ですか？"
```

**GPUオフロードを使用（例: 全レイヤー）:**

```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --prompt "Hello, world!" \
  --n-gpu-layers 99
```

### オプション

| オプション | 短縮形 | 説明 | デフォルト値 |
|------------|--------|------|--------------|
| `--model-path` | `-m` | **必須**: GGUFモデルファイルへのパス | - |
| `--prompt` | | **必須**: 生成を開始するためのプロンプト | - |
| `--max-tokens` | | 生成する最大トークン数 | 256 |
| `--n-gpu-layers` | | GPUにオフロードするレイヤー数 | 0 |

## ヘルプの表示

利用可能なすべてのオプションを確認するには、`--help` フラグを使用します。

```bash
cargo run -- --help
```
