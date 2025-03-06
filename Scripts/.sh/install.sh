#!/bin/sh

# Detect OS
ios="$(uname -s)"

if [ "$ios" = "Linux" ] || [ "$ios" = "Darwin" ]; then
    echo "Installing Rust on Unix-based system using VLM Rust CLI..."
    vlm rust install
elif [ "$os" = "Windows_NT" ]; then
    echo "Installing Rust on Windows via VLM Rust CLI..."
    powershell -Command "Start-Process -NoNewWindow -Wait -FilePath vlm -ArgumentList 'rust install'"
else
    echo "Unsupported OS: $ios"
    exit 1
fi

# Verify installation
if command -v rustc >/dev/null 2>&1; then
    echo "Rust installed successfully using VLM!"
    rustc --version
else
    echo "Rust installation failed. Please check for errors."
fi
