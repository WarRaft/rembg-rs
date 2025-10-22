#!/bin/bash

# Test script for rembg-rs

set -e

echo "🧪 Testing rembg-rs"
echo ""

# Check if binary exists
if [ ! -f "./target/release/rembg-rs" ]; then
    echo "❌ Binary not found. Run: cargo build --release"
    exit 1
fi

# Check if models directory exists
if [ ! -d "models" ]; then
    echo "⚠️  Models directory not found. Running download script..."
    ./download_model.sh
    echo ""
fi

# Check if u2net model exists
if [ ! -f "models/u2net.onnx" ]; then
    echo "❌ Model not found: models/u2net.onnx"
    echo "Run: ./download_model.sh"
    exit 1
fi

# Create test directories
mkdir -p test_input

# Clean output directory
echo "🧹 Cleaning test_output directory..."
rm -rf test_output
mkdir -p test_output
echo ""

# Find all images in test_input
shopt -s nullglob
VALID_IMAGES=()
for ext in jpg jpeg png JPG JPEG PNG; do
    for img in test_input/*."$ext"; do
        if [ -f "$img" ]; then
            VALID_IMAGES+=("$img")
        fi
    done
done
shopt -u nullglob

# Check if any images found
if [ ${#VALID_IMAGES[@]} -eq 0 ]; then
    echo "⚠️  No images found in test_input/"
    echo ""
    echo "Please add images to test_input/ directory:"
    echo "  Supported formats: jpg, jpeg, png"
    echo ""
    echo "You can use any image with a clear subject on a background."
    exit 1
fi

echo "📸 Found ${#VALID_IMAGES[@]} image(s) to process"
echo ""

# Process each image
COUNTER=0
for INPUT_IMAGE in "${VALID_IMAGES[@]}"; do
    COUNTER=$((COUNTER + 1))
    
    # Get filename without path
    FILENAME=$(basename "$INPUT_IMAGE")
    # Remove extension
    NAME="${FILENAME%.*}"
    # Output as PNG
    OUTPUT_IMAGE="test_output/${NAME}_no_bg.png"
    
    echo "[$COUNTER/${#VALID_IMAGES[@]}] Processing: $FILENAME"
    
    # Run background removal with library path set
    DYLD_LIBRARY_PATH="./target/release:$DYLD_LIBRARY_PATH" ./target/release/rembg-rs \
        -i "$INPUT_IMAGE" \
        -o "$OUTPUT_IMAGE" \
        -m "models/u2net.onnx"
    
    echo "    ✅ Saved: $OUTPUT_IMAGE"
    echo ""
done

echo "✅ All done! Processed ${#VALID_IMAGES[@]} image(s)"
echo ""
echo "Check results in: test_output/"
echo ""
echo "You can open the folder with:"
echo "  open test_output/"
