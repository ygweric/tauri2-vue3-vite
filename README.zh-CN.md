[English README](./README.md)

# tauri-vue-vite-template

一个结合了 **Tauri**、**Vue 3** 和 **Vite** 的现代桌面应用开发模板，市面上罕见的三合一解决方案。

## 🚀 核心特性
- ⚡ **Tauri**：基于 Rust 后端，构建轻量、安全且高效的桌面应用。
- 🖼️ **Vue 3**：现代响应式 UI 开发。
- 🛠️ **Vite**：极速构建工具与开发服务器。
- 🧩 开箱即用，最小化配置。
- 📦 适用于开源及商业项目。

## ✨ 功能描述

### 🔀 HTTP 接口转发
内置 HTTP 代理端点，可将 API 请求转发到任意目标服务器。可作为轻量级 API 网关或代理，适用于跨域请求、调试或多后端集成等场景。

### 🛰️ 自动端口检测
应用启动时会自动扫描并绑定本地可用端口。如果默认端口被占用，会自动递增查找下一个可用端口，确保启动无端口冲突。

## 🖥️ 开发环境
- Node.js >= 18.x
- pnpm >= 8.x（或 npm/yarn）
- Rust >= 1.70（用于 Tauri）
- Windows 10/11、macOS 或 Linux

## 📦 快速开始
```bash
# 安装依赖
pnpm install

# 启动开发环境
pnpm tauri dev

# 打包发布
pnpm tauri build
```

## 📄 许可证
MIT

---

> 本项目为开源桌面应用开发提供了现代化的起点，欢迎 Star & PR！ 