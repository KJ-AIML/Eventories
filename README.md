# 🎉 Eventories

> **Eventories** – A blazing-fast, Rust-powered event management platform.
> Upload. Search. Share. Celebrate.

Built with **Axum**, **ONNX Runtime**, and **Qdrant**, Eventories makes it easy to manage event photos, recognize faces, and let everyone find their memories instantly. 🚀

---

## ✨ Features

* 📸 **Smart Photo Uploads** – auto-detect and embed faces on upload
* 🧑‍🤝‍🧑 **Face Search** – find all your event shots with one selfie
* 🔗 **QR Event Codes** – join events effortlessly
* 📦 **Bulk Downloads** – export all or just your personal photos as ZIP
* 🔐 **Secure & Scalable** – Rust backend, async everything, vector search powered by Qdrant

---

## 🚀 Quick Start

```bash
# Development with hot reload
./run.sh dev

# Or with make
make dev
```

Once running:

* Root: [http://localhost:8080/](http://localhost:8080/)
* Health: [http://localhost:8080/health](http://localhost:8080/health)
* Docs (Scalar UI): [http://localhost:8080/scalar](http://localhost:8080/scalar)

---

## 📚 API Documentation

We ship **OpenAPI 3.0** out of the box.

* **Scalar UI** → [http://localhost:8080/scalar](http://localhost:8080/scalar)
* **OpenAPI JSON** → [http://localhost:8080/api-docs/openapi.json](http://localhost:8080/api-docs/openapi.json)

---

## 🛠 Development Workflow

```bash
# Install dev tools
make install-dev-tools

# Run in dev mode
make dev

# Run tests
make test

# Build release binary
make build

# Format code
make fmt
```

---

## 🐳 Docker Ready

```bash
# One-liner run
./run.sh prod

# Or manual
make docker-build
make docker-run
```

---

## 🗂 Project Structure

```
src/
├── adapter/          # Interface adapters
│   ├── inbound/      # HTTP routes & controllers
│   └── outbound/     # External service clients (Qdrant, storage, etc.)
├── application/      # Core use-cases
├── domain/           # Business logic (event, photo, face)
├── infrastructure/   # Config, DB pool, telemetry
└── shared/           # Utilities & common layers
```

---

## 🔧 Configuration

Environment variables:

| Key           | Default | Description                 |
| ------------- | ------- | --------------------------- |
| `PORT`        | 8080    | HTTP server port            |
| `ENVIRONMENT` | dev     | `dev` or `prod`             |
| `RUST_LOG`    | info    | Log level (info/debug/warn) |

---

## 🧭 Roadmap

* [ ] Social login (Google OAuth)
* [ ] Admin dashboard for event owners
* [ ] Live camera face matching
* [ ] Mobile-friendly photo browser

---

## 💡 Why Eventories?

Because memories deserve speed, security, and style.
Rust gives us **safety + performance**, ONNX brings **state-of-the-art AI**, and Qdrant ensures **search at scale**.

---

## 🤝 Contributing

Pull requests welcome 💌.
Run tests, lint, and make sure CI is green before submitting.

---

## 📜 License

MIT © Eventories Team
