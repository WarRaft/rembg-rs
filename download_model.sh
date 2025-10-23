#!/bin/bash

# Script to download ONNX models for background removal

set -e

# ============================================================================
# Configuration
# ============================================================================

# Directory where models will be stored
MODEL_DIR="models"

# Base URL for model downloads
BASE_URL="https://github.com/danielgatis/rembg/releases/download/v0.0.0"

# Model URLs and filenames
U2NET_URL="$BASE_URL/u2net.onnx"
U2NET_FILE="$MODEL_DIR/u2net.onnx"

U2NET_HUMAN_URL="$BASE_URL/u2net_human_seg.onnx"
U2NET_HUMAN_FILE="$MODEL_DIR/u2net_human_seg.onnx"

SILUETA_URL="$BASE_URL/silueta.onnx"
SILUETA_FILE="$MODEL_DIR/silueta.onnx"

# ============================================================================
# Download Functions
# ============================================================================

# Create models directory
mkdir -p "$MODEL_DIR"

echo "🚀 Starting model download..."
echo ""

# Download U2-Net - universal model (~176 MB)
if [ -f "$U2NET_FILE" ]; then
    echo "✅ u2net.onnx already exists"
else
    echo "📦 Downloading u2net.onnx (~176 MB)..."
    curl -L -o "$U2NET_FILE" "$U2NET_URL"
    echo "✅ u2net.onnx downloaded"
fi

echo ""

# Download U2-Net Human Segmentation - for portraits (~176 MB)
if [ -f "$U2NET_HUMAN_FILE" ]; then
    echo "✅ u2net_human_seg.onnx already exists"
else
    echo "📦 Downloading u2net_human_seg.onnx (~176 MB)..."
    curl -L -o "$U2NET_HUMAN_FILE" "$U2NET_HUMAN_URL"
    echo "✅ u2net_human_seg.onnx downloaded"
fi

echo ""

# Download Silueta - fast and lightweight model (~43 MB)
if [ -f "$SILUETA_FILE" ]; then
    echo "✅ silueta.onnx already exists"
else
    echo "📦 Downloading silueta.onnx (~43 MB)..."
    curl -L -o "$SILUETA_FILE" "$SILUETA_URL"
    echo "✅ silueta.onnx downloaded"
fi

if [ -f "$SILUETA_FILE" ]; then
    echo "✅ silueta.onnx already exists"
else
    echo "📦 Downloading silueta.onnx (~43 MB)..."
    curl -L -o "$SILUETA_FILE" "$SILUETA_URL"
    echo "✅ silueta.onnx downloaded"
fi

# ============================================================================
# Summary
# ============================================================================

echo ""
echo "✅ All models downloaded to: $MODEL_DIR/"
echo ""
echo "Usage examples:"
echo "  ./rembg-rs -i input.jpg -o output.png -m $MODEL_DIR/u2net.onnx"
echo "  ./rembg-rs -i photo.jpg -o result.png -m $MODEL_DIR/u2net_human_seg.onnx"
echo "  ./rembg-rs -i image.jpg -o output.png -m $MODEL_DIR/silueta.onnx"
