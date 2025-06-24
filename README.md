# 🌌 Astrix Kernel

*A simple yet ambitious hobby OS kernel built from scratch in Rust.*

![Astrix Banner](https://via.placeholder.com/800x200?text=Astrix+Kernel)

## 🚀 Overview

Astrix is the beating heart of **Aether OS** — a minimal, modular, and blazing-fast operating system designed with clarity, learning, and flexibility in mind. It began with a single `println!()` and has grown into a powerful custom-built kernel created by a 13-year-old dev with unstoppable curiosity.

## ✨ Features

- 🧠 Written entirely in Rust
- 🪵 Custom `print!` and `println!` macros
- ⚠️ Kernel panic handler
- 🧱 No_std + bare-metal build
- 💥 Boots with QEMU or real hardware
- 📁 Over 500 files of modular code (and counting)

## 📦 Build & Run

To build and launch Astrix:

```bash
# Install Rust nightly
rustup default nightly

# Add target
rustup target add x86_64-aether_os

# Build
cargo build --target x86_64-aether_os.json

# Run with QEMU
cargo run
