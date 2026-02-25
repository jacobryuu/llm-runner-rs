# LLM Runner RS - API仕様

## 概要
LLM Runner RSは、ローカルLLMモデル(GGUF形式)を実行するためのRust製REST APIサーバーです。

## エンドポイント

### ヘルスチェック
```
GET /health
```

**レスポンス:**
```json
{
  "status": "healthy",
  "model_loaded": true,
  "uptime_seconds": 3600
}
```

### チャット (非ストリーミング)
```
POST /v1/chat
```

**リクエスト:**
```json
{
  "prompt": "Rustプログラミング言語の利点は何ですか？",
  "max_tokens": 256,
  "temperature": 0.7,
  "top_p": 0.9,
  "stream": false
}
```

**レスポンス:**
```json
{
  "response": "Rustの主な利点は...",
  "tokens_generated": 150,
  "finish_reason": "stop"
}
```

### チャット (ストリーミング)
```
POST /v1/chat
```

**リクエスト:**
```json
{
  "prompt": "Rustプログラミング言語の利点は何ですか？",
  "max_tokens": 256,
  "stream": true
}
```

**レスポンス:** Server-Sent Events (SSE)
```
data: {"delta":"Rust","finish_reason":null}
data: {"delta":"の主な","finish_reason":null}
data: {"delta":"利点は","finish_reason":null}
...
data: {"delta":"","finish_reason":"stop"}
```

## パラメータ

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|-----------|-----|------|-----------|------|
| prompt | string | Yes | - | 入力プロンプト (1-4096文字) |
| max_tokens | integer | No | 256 | 生成する最大トークン数 (1-2048) |
| temperature | float | No | 0.7 | サンプリング温度 (0.0-2.0) |
| top_p | float | No | 0.9 | nucleus sampling (0.0-1.0) |
| stream | boolean | No | false | ストリーミングレスポンス |

## エラーレスポンス

```json
{
  "error": "Validation error: Prompt must be between 1 and 4096 characters",
  "code": 400
}
```

### ステータスコード

| コード | 説明 |
|--------|------|
| 200 | 成功 |
| 400 | バリデーションエラー |
| 408 | タイムアウト |
| 429 | レート制限超過 |
| 500 | 内部サーバーエラー |
| 503 | モデル読み込みエラー |

## 設定

### 環境変数
```bash
LLM__SERVER__HOST=127.0.0.1
LLM__SERVER__PORT=8080
LLM__MODEL__PATH=./models/model.gguf
LLM__INFERENCE__MAX_TOKENS=256
LLM__INFERENCE__TEMPERATURE=0.7
LLM__SECURITY__RATE_LIMIT_PER_MINUTE=60
```

### 設定ファイル (TOML)
`config/default.toml`, `config/production.toml`, `config/local.toml`

## 実行方法

### CLIモード
```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --prompt "Hello, world!"
```

### サーバーモード
```bash
cargo run --release -- \
  --model-path ./models/gemma-2-2b-it-Q4_K_M.gguf \
  --server-mode
```

### curlでのテスト
```bash
# ヘルスチェック
curl http://localhost:8080/health

# チャット (非ストリーミング)
curl -X POST http://localhost:8080/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Rustプログラミング言語の利点は何ですか？",
    "max_tokens": 100
  }'

# チャット (ストリーミング)
curl -N -X POST http://localhost:8080/v1/chat \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Rustプログラミング言語の利点は何ですか？",
    "stream": true
  }'
```

## セキュリティ機能

- レート制限: デフォルト60リクエスト/分
- リクエストサイズ制限: 1MB
- 入力バリデーション
- タイムアウト設定: デフォルト300秒
- 同時リクエスト制限: デフォルト10

## 監視とログ

### ログレベル
```bash
# 環境変数で設定
RUST_LOG=info cargo run --release

# レベル: trace, debug, info, warn, error
```

### 構造化ログ
- すべてのリクエストをトレース
- エラーの詳細ログ
- パフォーマンスメトリクス
