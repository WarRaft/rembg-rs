#!/bin/bash

# Script to download ONNX models for background removal

set -e

MODEL_DIR="models"
DOWNLOAD_DIR="$MODEL_DIR"

# Create models directory
mkdir -p "$DOWNLOAD_DIR"

echo "🚀 Starting model download..."
echo ""

# U2-Net - universal model
U2NET_URL="https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx"
U2NET_FILE="$DOWNLOAD_DIR/u2net.onnx"

if [ -f "$U2NET_FILE" ]; then
    echo "✅ u2net.onnx already exists"
else
    echo "📦 Downloading u2net.onnx (~176 MB)..."
    curl -L -o "$U2NET_FILE" "$U2NET_URL"
    echo "✅ u2net.onnx downloaded"
fi

echo ""

# U2-Net Human Segmentation - for portraits
U2NET_HUMAN_URL="https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_human_seg.onnx"
U2NET_HUMAN_FILE="$DOWNLOAD_DIR/u2net_human_seg.onnx"

if [ -f "$U2NET_HUMAN_FILE" ]; then
    echo "✅ u2net_human_seg.onnx already exists"
else
    echo "📦 Downloading u2net_human_seg.onnx (~176 MB)..."
    curl -L -o "$U2NET_HUMAN_FILE" "$U2NET_HUMAN_URL"
    echo "✅ u2net_human_seg.onnx downloaded"
fi

echo ""

# Silueta - fast and lightweight model
SILUETA_URL="https://github.com/danielgatis/rembg/releases/download/v0.0.0/silueta.onnx"
SILUETA_FILE="$DOWNLOAD_DIR/silueta.onnx"

if [ -f "$SILUETA_FILE" ]; then
    echo "✅ silueta.onnx already exists"
else
    echo "📦 Downloading silueta.onnx (~43 MB)..."
    curl -L -o "$SILUETA_FILE" "$SILUETA_URL"
    echo "✅ silueta.onnx downloaded"
fi

echo ""
echo "✅ All models downloaded to: $MODEL_DIR/"
echo ""
echo "Usage:"
echo "  ./target/release/rembg-rs -i input.jpg -o output.png -m models/u2net.onnx"
