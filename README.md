[简体中文说明](./README.zh-CN.md)

# tauri-vue-vite-template

A modern template combining **Tauri**, **Vue 3**, and **Vite** for building cross-platform desktop applications.

## 🚀 Core Features
- ⚡ **Tauri**: Build lightweight, secure, and fast desktop apps with Rust backend.
- 🖼️ **Vue 3**: Modern, reactive UI development.
- 🛠️ **Vite**: Lightning-fast build tool and dev server.
- 🧩 Out-of-the-box integration, minimal setup.
- 📦 Ready for open source and commercial projects.

## ✨ Features Description

### 🔀 HTTP API Forwarding
Built-in HTTP proxy endpoint for forwarding API requests to any target server. This enables the app to act as a lightweight API gateway or proxy, useful for cross-origin requests, debugging, or integrating with multiple backends.

### 🛰️ Automatic Port Detection
On startup, the app automatically scans and binds to an available local port. If the default port is occupied, it will incrementally search for the next available port, ensuring seamless startup without port conflicts.

## 🖥️ Development Environment
- Node.js >= 18.x
- pnpm >= 8.x (or npm/yarn)
- Rust >= 1.70 (for Tauri)
- Windows 10/11, macOS, or Linux

## 📦 Getting Started
```bash
# Install dependencies
pnpm install

# Start development
pnpm tauri dev

# Build for production
pnpm tauri build
```

## 📄 License
MIT

---

> This project provides a modern starting point for open source desktop app development. Star & PR are welcome!
