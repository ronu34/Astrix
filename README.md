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


## 📦 Build & Run

To build and launch Astrix:

```bash
#Install Rust(If you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Clone this repo
git clone https://github.com/ronu34/Astrix.git

# Install Rust nightly
rustup default nightly

# Run with QEMU
cargo run
