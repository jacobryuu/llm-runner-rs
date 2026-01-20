#!/bin/bash

# 引数が指定されていない場合、使い方を表示して終了
if [ -z "$1" ]; then
    echo "Usage: $0 <model_name>"
    echo ""
    echo "Available models:"
    echo "  tinyllama   (TinyLlama-1.1B, ~0.7GB) - Very lightweight, basic performance."
    echo "  gemma2b     (Gemma 2 2B, ~1.7GB) - Recommended for good balance of performance and size."
    echo "  qwen3b      (Qwen 2.5 3B, ~2.0GB) - High quality, especially for Japanese."
    exit 1
fi

MODEL_NAME=$1
MODEL_URL=""
OUTPUT_FILE=""

# モデル名に応じてURLとファイル名を設定
case $MODEL_NAME in
  tinyllama)
    MODEL_URL="https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"
    OUTPUT_FILE="models/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"
    ;;
  gemma2b)
    MODEL_URL="https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf"
    OUTPUT_FILE="models/gemma-2-2b-it-Q4_K_M.gguf"
    ;;
  qwen3b)
    # Qwen 2.5 3B Instruct (Q4_K_M) - Size: ~1.93 GB
    MODEL_URL="https://huggingface.co/bartowski/Qwen2.5-3B-Instruct-GGUF/resolve/main/Qwen2.5-3B-Instruct-Q4_K_M.gguf"
    OUTPUT_FILE="models/Qwen2.5-3B-Instruct-Q4_K_M.gguf"
    ;;
  *)
    echo "Error: Unknown model '$MODEL_NAME'"
    echo "Available models: tinyllama, gemma2b, qwen3b"
    exit 1
    ;;
esac

# モデル保存用ディレクトリの作成
mkdir -p models

echo "Downloading $MODEL_NAME model..."
echo "URL: $MODEL_URL"
echo "Output: $OUTPUT_FILE"

# curlまたはwgetを使用してダウンロード
if command -v curl >/dev/null 2>&1; then
    curl -L -o "$OUTPUT_FILE" "$MODEL_URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O "$OUTPUT_FILE" "$MODEL_URL"
else
    echo "Error: curl or wget is required to download the model."
    exit 1
fi

echo ""
echo "Download complete!"
echo "You can now run the inference engine with:"
echo "cargo run --release -- --model-path $OUTPUT_FILE --prompt \"<Your prompt here>\""
