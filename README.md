# Simple HTTP Server Library

A lightweight and straightforward HTTP server library written in Rust. This library is designed to handle basic HTTP requests and responses using minimal external dependencies, only relying on `log` and `anyhow`.

## Features

- Lightweight HTTP server implementation.
- Simple request routing with support for multiple HTTP methods.
- Uses only `log` for logging and `anyhow` for error handling.
- Thread-safe argument handling with `Arc<RwLock<Args>>`.

## Getting Started

### Prerequisites

- Rust (version 1.47 or higher)

### Installation

Add the following to your `Cargo.toml`:

[dependencies]
fobserver = "0.1.0"
