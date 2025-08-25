# eventories

Modern Rust microservice with Clean Architecture.

## 🚀 Quick Start

```bash
# Development with hot reload
./run.sh dev

# Or using make
make dev
```

## 📡 Endpoints

- `GET /` - Root endpoint
- `GET /health` - Health check
- `GET /api/v1/info` - API info endpoint
- `GET /scalar` - API documentation (Scalar UI)

## 📚 API Documentation

API documentation is available via Scalar UI:
- **Scalar UI**: `http://localhost:8080/scalar`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

## 🛠️ Development

```bash
# Install dev tools
make install-dev-tools

# Run with hot reload
make dev

# Run tests
make test

# Build release
make build

# Format code
make fmt
```

## 🐳 Docker

```bash
# Build and run with Docker
./run.sh prod

# Or manually
make docker-build
make docker-run
```

## 📦 Project Structure

```
src/
├── adapter/          # Interface adapters
│   ├── inbound/      # HTTP handlers
│   └── outbound/     # External services
├── application/      # Use cases
├── domain/          # Business logic
├── infrastructure/  # Configuration
└── shared/          # Common utilities
```

## 🔧 Configuration

Environment variables:
- `PORT` - Server port (default: 8080)
- `ENVIRONMENT` - Environment (development/production)
- `RUST_LOG` - Log level (info/debug/error)
