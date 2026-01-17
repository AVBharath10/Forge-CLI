# Forge 

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

**Forge** is an opinionated environment and configuration generator for modern web applications. It helps you bootstrap the infrastructure configuration for your projects quickly and consistently.

##  Features

- **Interactive CLI**: Easy-to-use dialog-based interface.
- **Frontend**: Next.js (App Router), React (Vite).
- **Backend**: Express (Node.js), FastAPI (Python).
- **Database**: Postgres, MySQL, MongoDB, SQLite.
- **Auth**: Built-in JWT scaffolding with Login/Register routes.
- **DevOps**: Auto-generated `docker-compose.yml` and GitHub Actions CI.

##  Installation

### From Components (Prerequisite)
Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

### Building from Source

```bash
git clone https://github.com/AVBharath10/Forge-CLI.git
cd forge
cargo install --path .
```

## Usage

Run the `init` command to start the interactive generator:

```bash
forge init
```

### Generated Stack
Running `forge init` will create a directory containing:
- **Frontend**: A fully configure Web App.
- **Backend**: Server with pre-configured DB connection and Auth.
- **Infrastructure**: CI/CD workflows and Docker Compose files.
- **.env**: A pre-filled `.env` file (and `.env.example`).

##  Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

##  License

Distributed under the MIT License. See `LICENSE` for more information.
