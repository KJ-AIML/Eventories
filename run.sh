#!/bin/bash

case "$1" in
    "dev")
        echo "🚀 Starting development server with hot reload..."
        if ! command -v cargo-watch &> /dev/null; then
            echo "Installing cargo-watch for hot reload..."
            cargo install cargo-watch
        fi
        cargo watch -x run
        ;;
    "dev-docker")
        echo "🐳 Starting with Docker..."
        docker-compose -f docker-compose.dev.yaml up --build
        ;;

    "prod")
        echo "🐳 Starting with Docker..."
        docker-compose -f docker-compose.prod.yaml up --build
        ;;
    "build")
        echo "🔨 Building release..."
        cargo build --release
        ;;
    *)
        echo "Usage: $0 {dev|prod|build}"
        echo ""
        echo "Commands:"
        echo "  dev     - Start development server with hot reload"
        echo "  prod    - Start with Docker"
        echo "  build   - Build release binary"
        echo ""
        echo "Quick start:"
        echo "  ./run.sh dev"
        ;;
esac
