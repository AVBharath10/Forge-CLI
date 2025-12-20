# Forge

Forge is an opinionated environment and configuration generator for modern web applications. It helps you bootstrap the infrastructure configuration for your projects quickly and consistently.

## Features

- 🚀 **Interactive CLI**: Easy-to-use command-line interface for selecting your project stack.
- ⚡ **Framework Support**: Built-in configuration generation for **Next.js** and **Express**.
- 🔒 **Authentication Options**: Optional **JWT** setup and environment variable generation.
- 🐳 **Docker-Ready**: Automatically generates `docker-compose.yml` for a local **PostgreSQL** database.
- ⚙️ **Environment configs**: Creates `.env.example` tailored to your selected stack.

## Installation

### Prerequisites

- Rust and Cargo installed on your machine.

### Building from Source

```bash
git clone https://github.com/yourusername/forge.git
cd forge
cargo install --path .
```

## Usage

Run the `init` command to start the interactive generator:

```bash
forge init
```

### What it generates

Running `forge init` will create a `forge-project` directory containing:

- `docker-compose.yml`: PostgreSQL service configuration.
- `.env.example`: Template for environment variables.
- `README.md`: Project-specific instructions.

## Development

To run the project locally during development:

```bash
cargo run -- init
```
