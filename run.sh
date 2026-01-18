#!/bin/bash

# AutoClicker Setup Script

echo "AutoClicker - Rust Implementation"
echo "=================================="
echo ""
echo "To run the application, you need to install system dependencies first:"
echo ""
echo "On Arch-based systems (like CachyOS):"
echo "  sudo pacman -S xdotool libxtst libxinerama libxcursor libxrandr libxss"
echo ""
echo "On Ubuntu/Debian-based systems:"
echo "  sudo apt-get install xdotool libxtst-dev libxinerama-dev libxcursor-dev libxrandr-dev libxss-dev"
echo ""
echo "On Fedora/RHEL-based systems:"
echo "  sudo dnf install xdotool libXtst-devel libXinerama-devel libXcursor-devel libXrandr-devel libXScrnSaver-devel"
echo ""
echo "After installing dependencies, run the application with:"
echo "  cargo run --release"
echo ""
echo "Or build it first:"
echo "  cargo build --release"
echo "  ./target/release/autoclicker"
