# 🌌 Astrix Kernel

*A simple yet ambitious hobby OS kernel built from scratch in Rust.*


## 🚀 Overview

Astrix is the beating heart of **Aether OS** — a minimal, modular, and blazing-fast operating system designed with clarity, learning, and flexibility in mind. It only has println! , print!, write! working.

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
