.PHONY: dev build test clean docker-build docker-run

# Development with hot reload
dev:
	@echo "🚀 Starting development server with hot reload..."
	@cargo watch -x run

# Build release
build:
	@echo "🔨 Building release..."
	@cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@cargo clean

# Docker build
docker-build:
	@echo "🐳 Building Docker image..."
	@docker build -t eventories .

# Docker run
docker-run: docker-build
	@echo "🐳 Running Docker container..."
	@docker run -p 8080:8080 eventories

# Install cargo-watch for hot reload
install-dev-tools:
	@echo "📦 Installing development tools..."
	@cargo install cargo-watch

# Format code
fmt:
	@cargo fmt

# Check code
check:
	@cargo check

# Run with logs
run-debug:
	@RUST_LOG=debug cargo run
