#!/bin/bash

# Exit on error
set -e

echo "🚀 Starting setup for Taian's CV & Personal Hub..."

# 1. Ensure cargo is in PATH
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/env" ]; then
        echo "🌐 Sourcing $HOME/.cargo/env..."
        . "$HOME/.cargo/env"
    fi
fi

# Final check for cargo
if ! command -v cargo &> /dev/null; then
    # Fallback to direct path check
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    else
        echo "❌ Error: cargo is not installed. Please install Rust first: https://www.rust-lang.org/tools/install"
        exit 1
    fi
fi
echo "✅ cargo found: $(command -v cargo)"

# 2. Install cargo-binstall if missing (helps speed up other installs)
if ! command -v cargo-binstall &> /dev/null; then
    echo "📦 Installing cargo-binstall..."
    cargo install cargo-binstall
else
    echo "✅ cargo-binstall is already installed."
fi

# 3. Install dioxus-cli
if ! command -v dx &> /dev/null; then
    echo "🔌 Installing dioxus-cli..."
    cargo binstall -y dioxus-cli
else
    echo "✅ dioxus-cli (dx) is already installed."
fi

# 4. Install prek
if ! command -v prek &> /dev/null; then
    echo "⚓ Installing prek..."
    cargo binstall -y prek
else
    echo "✅ prek is already installed."
fi

# 5. Set up git hooks
echo "🔧 Activating git hooks..."
prek install

# 6. Set up environment variables
if [ ! -f .env ]; then
    echo "📄 Creating .env from .env.template..."
    cp .env.template .env
    echo "⚠️  ACTION REQUIRED: Please update the .env file with your Google OAuth credentials."
else
    echo "✅ .env file already exists."
fi

echo ""
echo "✨ Setup complete! ✨"
echo "To start the development server, run:"
echo "  export \$(cat .env | xargs) && dx serve"
echo ""
