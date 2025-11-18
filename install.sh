#!/bin/bash

# ResMan TUI - Resource Monitor Installer
# Cross-platform installation script for Linux/macOS

set -e  # Exit on error

echo "================================================"
echo "   ResMan TUI - Resource Monitor Installer"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     PLATFORM=Linux;;
    Darwin*)    PLATFORM=Mac;;
    *)          PLATFORM="UNKNOWN:${OS}"
esac

echo "Detected platform: $PLATFORM"
echo ""

# Check if Rust is installed
echo "[1/4] Checking for Rust installation..."
if ! command -v cargo &> /dev/null; then
    echo ""
    echo "${YELLOW}Rust is not installed!${NC}"
    echo ""
    echo "Installing Rust..."
    echo "Please follow the installer instructions."
    echo ""

    # Download and run rustup installer
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Source cargo env
    source "$HOME/.cargo/env"

    echo ""
    echo "${GREEN}Rust installation complete!${NC}"
else
    cargo --version
    echo "${GREEN}Rust is already installed!${NC}"
fi

echo ""
echo "[2/4] Building ResMan TUI..."
echo "This may take a few minutes on first build..."
echo ""

# Ask user if they want GPU support
read -p "Do you want GPU support (NVIDIA only)? (y/n): " GPU_SUPPORT

if [[ "$GPU_SUPPORT" =~ ^[Yy]$ ]]; then
    echo "Building with NVIDIA GPU support..."

    # Check if NVIDIA drivers are available
    if command -v nvidia-smi &> /dev/null; then
        echo "${GREEN}NVIDIA drivers detected${NC}"

        if cargo build --release --features gpu-nvidia; then
            echo "${GREEN}Build with GPU support successful!${NC}"
            BUILD_FEATURES="--features gpu-nvidia"
        else
            echo ""
            echo "${YELLOW}WARNING: Build with GPU support failed!${NC}"
            echo "Make sure you have NVIDIA drivers and development libraries installed."
            echo ""
            echo "For Ubuntu/Debian: sudo apt-get install libnvidia-ml-dev"
            echo "For Fedora/RHEL:   sudo dnf install nvidia-driver-devel"
            echo ""
            echo "Falling back to build without GPU support..."
            cargo build --release
            BUILD_FEATURES=""
        fi
    else
        echo "${YELLOW}WARNING: NVIDIA drivers not detected. Building without GPU support.${NC}"
        cargo build --release
        BUILD_FEATURES=""
    fi
else
    echo "Building without GPU support..."
    cargo build --release
    BUILD_FEATURES=""
fi

echo ""
echo "[3/4] Installing ResMan globally..."
echo ""

if [ -n "$BUILD_FEATURES" ]; then
    cargo install --path . $BUILD_FEATURES --force
else
    cargo install --path . --force
fi

echo ""
echo "[4/4] Verifying installation..."
echo ""

# Check if resmon is in PATH
if command -v rtui &> /dev/null; then
    echo "${GREEN}Installation successful!${NC}"
    rtui --version 2>/dev/null || echo "rtui installed successfully!"
else
    echo ""
    echo "${YELLOW}WARNING: 'rtui' not found in PATH!${NC}"
    echo ""
    echo "Please add the following to your PATH:"
    echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
    echo "Add this line to your shell config:"
    if [ "$PLATFORM" = "Mac" ]; then
        echo "  echo 'export PATH=\"\$HOME/.cargo/bin:\$PATH\"' >> ~/.zshrc"
        echo "  source ~/.zshrc"
    else
        echo "  echo 'export PATH=\"\$HOME/.cargo/bin:\$PATH\"' >> ~/.bashrc"
        echo "  source ~/.bashrc"
    fi
    echo ""
fi

echo ""
echo "================================================"
echo "          Installation Complete!"
echo "================================================"
echo ""
echo "To run ResMan TUI, simply type:"
echo "    ${GREEN}rtui${NC}"
echo ""
echo "Controls:"
echo "  - Up/Down or j/k   : Navigate processes"
echo "  - Enter            : View process details"
echo "  - Esc              : Close details"
echo "  - q                : Quit"
echo ""
echo "If 'rtui' command is not found, restart your terminal"
echo "or add \$HOME/.cargo/bin to your PATH."
echo ""
