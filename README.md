# Morr: My Own Redis in Rust

[![Rust](https://img.shields.io/badge/rust-1.84+-blue.svg)](https://www.rust-lang.org/)

Morr is a simple in-memory key-value store database implemented in Rust and strongly inspired by Redis. This project was created for **learning purposes**.

---

## Features

- **In-Memory Storage**: Stores key-value pairs in memory using a `HashMap`.
- **Basic Commands**:
  - `PING`: Responds with `PONG`.
  - `SET <key> <value>`: Stores a key-value pair.
  - `GET <key>`: Retrieves the value for a key.
  - `DEL <key>`: Deletes a key-value pair.
  - `EXIT`: Exits the CLI.
- **Simple CLI**: Interactive command-line interface for executing commands.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Build the project:
   ```bash
   cargo build
   ```

2. Run the program:
   ```bash
   cargo run
   ```

---

## Project Structure

```plaintext
morr/
├── Cargo.toml          # Project metadata and dependencies
├── src/                # Source code
│   ├── main.rs         # Entry point and CLI logic
│   └── commands.rs     # Command implementations
└── README.md           # Project documentation
```

---

## Acknowledgments

- Inspired by [Redis](https://redis.io/), an in-memory key-value store.
- Built with [Rust](https://www.rust-lang.org/), a systems programming language focused on safety and performance.

---
