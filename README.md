# Forge

Forge is an opinionated environment and configuration generator for modern web applications. It helps you bootstrap the infrastructure configuration for your projects quickly and consistently.

## Features

- **Interactive CLI**: Easy-to-use dialog-based interface for selecting your project stack.
- **Frontend Frameworks**: 
    - **Next.js**
    - **React (Vite)**
- **Backend Frameworks**: 
    - **Express** (Node.js)
    - **FastAPI** (Python)
- **Database Support**: 
    - **Postgres**
    - **MySQL**
    - **MongoDB**
    - **SQLite**
- **Authentication Scaffolding**: 
    - Optional **JWT Auth** generation.
    - Generates real authentication code: routes (`/login`, `/register`), middleware, and token handling logic for both Express and FastAPI.
- **CI/CD Ready**: 
    - Automatically generates a GitHub Actions workflow (`.github/workflows/ci.yml`) tailored to your chosen stack.
- **One-Command Setup**: 
    - Generates project structure (`frontend/`, `backend/`), `package.json` / `requirements.txt`, database connections, and `.env` files.
- **Docker**: Automatically generates `docker-compose.yml` for your selected database.

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

Running `forge init` will create a directory with your project name containing:

- **Frontend**: A fully set up Next.js or Vite React app.
- **Backend**: An Express or FastAPI server with:
    - **DB Connection**: Pre-configured connection code to your chosen database.
    - **Auth**: (If selected) Login and Register endpoints with JWT implementation.
    - **Dependencies**: All necessary drivers and libraries installed.
- **CI/CD**: A `.github/workflows/ci.yml` file for automated testing.
- **Docker**: A `docker-compose.yml` file to spin up your database locally.
- **.env**: A pre-filled `.env` file (and `.env.example`).

## Development

To run the project locally during development:

```bash
cargo run -- init
```
