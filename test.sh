#!/bin/bash

# Test script for rembg-rs - batch process all images in test_input/

set -e

# ============================================================================
# Configuration
# ============================================================================

# Directories (relative to repo root)
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
INPUT_DIR="$REPO_ROOT/test_input"
OUTPUT_DIR="$REPO_ROOT/test_output"
MODEL_DIR="$REPO_ROOT/models"

# Binary and model paths
BINARY_PATH="$REPO_ROOT/target/release/rembg-rs"
MODEL_FILE="$MODEL_DIR/u2net.onnx"

# Supported image extensions
IMAGE_EXTENSIONS=("jpg" "jpeg" "png" "JPG" "JPEG" "PNG")

# Output file suffix
OUTPUT_SUFFIX="_no_bg"

# Mask threshold (0–255) – higher values = more aggressive background removal
# Recommended values:
# 76–102: Soft edges with semi-transparency (natural blending)
# 128: Balanced (default)
# 153–179: Stronger cutout, cleaner edges
# 204+: Very aggressive removal (may cut into object)
THRESHOLD="160"


# Binary mode (true/false) - if true, creates hard cutout without semi-transparency
# Recommended: true for clean cutouts, false for natural edges
# true: Clean cutout with no semi-transparent pixels (best for art/icons)
# false: Soft edges for more natural blending (best for photos)
BINARY_MODE=true

# Save mask (true/false) - if true, saves the mask as a separate grayscale image
# Default: false (only save result)
# true: Save mask alongside result (useful for debugging and adjustments)
# false: Only save the result image
SAVE_MASK=true

# ============================================================================
# Validation
# ============================================================================

echo "🧪 Testing rembg-rs"
echo ""

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found at: $BINARY_PATH"
    echo "Run: ./test-build.sh or cargo build --release"
    exit 1
fi

# Check if models directory exists
# If models directory is missing, attempt to download
if [ ! -d "$MODEL_DIR" ]; then
    echo "⚠️  Models directory not found. Running download script..."
    "$REPO_ROOT/download_model.sh"
    echo ""
fi

# Check if model file exists
if [ ! -f "$MODEL_FILE" ]; then
    echo "❌ Model not found: $MODEL_FILE"
    echo "Run: ./download_model.sh"
    exit 1
fi

# ============================================================================
# Preparation
# ============================================================================

# Create input directory if it doesn't exist
mkdir -p "$INPUT_DIR"

# Clean and recreate output directory
echo "🧹 Cleaning $OUTPUT_DIR directory..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
echo ""

# Find all images in input directory
shopt -s nullglob
VALID_IMAGES=()
for ext in "${IMAGE_EXTENSIONS[@]}"; do
    for img in "$INPUT_DIR"/*."$ext"; do
        if [ -f "$img" ]; then
            VALID_IMAGES+=("$img")
        fi
    done
done
shopt -u nullglob

# Check if any images found
if [ ${#VALID_IMAGES[@]} -eq 0 ]; then
    echo "⚠️  No images found in $INPUT_DIR/"
    echo ""
    echo "Please add images to $INPUT_DIR/ directory"
    echo "Supported formats: ${IMAGE_EXTENSIONS[*]}"
    echo ""
    exit 1
fi

# ============================================================================
# Processing
# ============================================================================

echo "📸 Found ${#VALID_IMAGES[@]} image(s) to process"
echo ""

COUNTER=0
for INPUT_IMAGE in "${VALID_IMAGES[@]}"; do
    COUNTER=$((COUNTER + 1))
    
    # Get filename without path
    FILENAME=$(basename "$INPUT_IMAGE")
    
    # Remove extension
    NAME="${FILENAME%.*}"
    
    # Generate output filename (always PNG for transparency support)
    OUTPUT_IMAGE="$OUTPUT_DIR/${NAME}${OUTPUT_SUFFIX}.png"
    
    echo "[$COUNTER/${#VALID_IMAGES[@]}] Processing: $FILENAME"
    
    # Build command with optional parameters
    CMD_ARGS=(-i "$INPUT_IMAGE" -o "$OUTPUT_IMAGE" -m "$MODEL_FILE")
    
    # Add threshold parameter
    CMD_ARGS+=(-t "$THRESHOLD")
    
    # Add binary mode if enabled
    if [ "$BINARY_MODE" = true ]; then
        CMD_ARGS+=(-b)
    fi
    
    # Add save mask if enabled
    if [ "$SAVE_MASK" = true ]; then
        CMD_ARGS+=(-s)
    fi
    
    # Run background removal with library path set to the release dir
    DYLD_LIBRARY_PATH="$REPO_ROOT/target/release:$DYLD_LIBRARY_PATH" "$BINARY_PATH" "${CMD_ARGS[@]}"
    
    echo "    ✅ Saved: $OUTPUT_IMAGE"
    echo ""
done

# ============================================================================
# Summary
# ============================================================================

echo "✅ All done! Processed ${#VALID_IMAGES[@]} image(s)"
echo ""
echo "Check results in: $OUTPUT_DIR/"
echo ""
echo "You can open the folder with:"
echo "  open $OUTPUT_DIR/"
