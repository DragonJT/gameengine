#!/bin/bash
set -e  # Exit on error

C_PROJECT_DIR="core/cengine"
BUILD_DIR="$C_PROJECT_DIR/build"
EXEC_NAME="cengine"

echo "==> Creating build directory..."
mkdir -p "$BUILD_DIR"

echo "==> Running CMake..."
cmake -S "$C_PROJECT_DIR" -B "$BUILD_DIR"

echo "==> Building the project..."
cmake --build "$BUILD_DIR"

#echo "==> Running executable..."
#./"$BUILD_DIR"/"$EXEC_NAME"
