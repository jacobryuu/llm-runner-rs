# LLM Runner RS

RustによるプロダクションレディなLLM（Large Language Model）推論ランナーです。`llama.cpp`のRustバインディングを使用しており、GGUF形式のモデルをサポートしています。

## 特徴

### 🚀 実業務向け機能
- **REST API**: 完全な非同期REST APIサーバー（Axum使用）
- **ストリーミング対応**: Server-Sent Events (SSE)による低レイテンシ応答
- **並行処理**: 複数リクエストの同時処理対応
- **設定管理**: 環境変数・TOMLファイルによる柔軟な設定
- **ロギング**: 構造化ログ（tracing）とリクエストトレーシング
- **エラーハンドリング**: 詳細なエラー型とHTTPステータスコード
- **セキュリティ**: レート制限、入力バリデーション、タイムアウト
- **監視**: ヘルスチェックエンドポイント、メトリクス収集
- **テスト**: ユニットテスト・統合テスト

### 💻 動作モード
- **CLIモード**: インタラクティブなチャットインターフェース
- **サーバーモード**: REST APIサーバー

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

### CLIモード（インタラクティブチャット）

```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --prompt "Rustプログラミング言語の利点は何ですか？"
```

対話型セッション:
```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf
```

### サーバーモード（REST API）

```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --server-mode
```

ログレベル付きで起動:
```bash
RUST_LOG=info cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --server-mode
```

### APIの使用

#### ヘルスチェック
```bash
curl http://localhost:8080/health
```

#### チャット（非ストリーミング）
```bash
curl -X POST http://localhost:8080/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Rustプログラミング言語の利点は何ですか？",
    "max_tokens": 256,
    "temperature": 0.7
  }'
```

#### チャット（ストリーミング）
```bash
curl -N -X POST http://localhost:8080/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Rustプログラミング言語の利点は何ですか？",
    "stream": true
  }'
```

## 設定

### コマンドラインオプション

| オプション | 短縮形 | 説明 | デフォルト値 |
|------------|--------|------|--------------|
| `--model-path` | `-m` | **必須**: GGUFモデルファイルへのパス | - |
| `--prompt` | | 開始プロンプト（オプション） | - |
| `--max-tokens` | | 生成する最大トークン数 | 256 |
| `--n-gpu-layers` | | GPUにオフロードするレイヤー数 | 0 |
| `--server-mode` | | サーバーモードで起動 | false |

### 環境変数

`.env`ファイルまたは環境変数で設定:
```bash
LLM__SERVER__HOST=127.0.0.1
LLM__SERVER__PORT=8080
LLM__MODEL__PATH=./models/gemma-2-2b-it-Q4_K_M.gguf
LLM__INFERENCE__MAX_TOKENS=256
LLM__INFERENCE__TEMPERATURE=0.7
LLM__SECURITY__RATE_LIMIT_PER_MINUTE=60
```

### 設定ファイル（TOML）

`config/default.toml`でデフォルト設定を定義:
```toml
[server]
host = "127.0.0.1"
port = 8080
timeout_seconds = 300

[model]
path = "./models/gemma-2-2b-it-Q4_K_M.gguf"
context_size = 2048
gpu_layers = 0

[inference]
max_tokens = 256
temperature = 0.7
top_p = 0.9
repeat_penalty = 1.1

[security]
rate_limit_per_minute = 60
max_prompt_length = 4096
max_concurrent_requests = 10
```

環境別設定:
- `config/development.toml`
- `config/production.toml`
- `config/local.toml` (gitignore推奨)

`RUN_MODE`環境変数で切り替え:
```bash
RUN_MODE=production cargo run --release
```

## セキュリティ機能

- **レート制限**: デフォルト60リクエスト/分
- **リクエストサイズ制限**: 1MB
- **入力バリデーション**: 自動的なプロンプト検証
- **タイムアウト**: 長時間実行の防止（デフォルト300秒）
- **同時実行制限**: リソース保護（デフォルト10並行）

## 監視とログ

### ログレベル
```bash
RUST_LOG=trace  # 最も詳細
RUST_LOG=debug
RUST_LOG=info   # 推奨（デフォルト）
RUST_LOG=warn
RUST_LOG=error  # エラーのみ
```

### モジュール別ログ
```bash
RUST_LOG=llm_runner_rs=debug,tower_http=debug
```

## テスト

```bash
# すべてのテストを実行
cargo test

# 統合テストのみ
cargo test --test integration_test

# ログ出力付きでテスト
RUST_LOG=debug cargo test -- --nocapture
```

## ドキュメント

- [API仕様](docs/API.md) - REST APIの詳細仕様
- [アーキテクチャ](docs/ARCHITECTURE.md) - システム設計と構成

## プロジェクト構造

```
llm-runner-rs/
├── src/
│   ├── main.rs           # エントリーポイント
│   ├── lib.rs            # ライブラリクレート
│   ├── config.rs         # CLIオプション
│   ├── settings.rs       # アプリケーション設定
│   ├── error.rs          # エラー型定義
│   ├── engine.rs         # 推論エンジン
│   ├── service.rs        # ビジネスロジック
│   └── api/              # REST API
│       ├── mod.rs        # ルーター設定
│       ├── handlers.rs   # リクエストハンドラ
│       └── models.rs     # API型定義
├── config/               # 設定ファイル
│   └── default.toml
├── tests/                # 統合テスト
├── docs/                 # ドキュメント
└── models/               # モデルファイル
```

## パフォーマンス

- 非同期処理による高スループット
- セマフォによるリソース管理
- ストリーミングによる低レイテンシ
- GPUオフロード対応（Metal、CUDA）

## トラブルシューティング

### ビルドエラー
```bash
# 依存関係のクリーンビルド
cargo clean
cargo build --release
```

### モデル読み込みエラー
- モデルパスが正しいか確認
- GGUF形式であることを確認
- ファイル権限を確認

### メモリ不足
- `context_size`を削減
- `max_concurrent_requests`を削減
- より小さいモデルを使用

## ライセンス

MIT License - 詳細は[LICENSE](LICENSE)を参照

## 貢献

プルリクエストを歓迎します。大きな変更の場合は、まずissueを開いて変更内容を議論してください。
