# Forge

Forge is an opinionated environment and configuration generator for modern web applications.

It exists to answer a very specific question:

> “Why am I spending 45 minutes setting up the same project structure again?”

Forge helps you bootstrap infrastructure, tooling, and configuration quickly and consistently, so you can move on to writing actual application logic.

---

## What Forge Does

Forge generates a complete, ready-to-run project setup based on your selected stack.  
No placeholders, no TODOs, no “you figure this part out later”.

You answer a few questions.
Forge scaffolds the rest.

---

## Features

### Interactive CLI

Forge uses an interactive, dialog-based CLI to guide you through stack selection without requiring you to memorize flags or templates.

---

### Frontend Frameworks

- Next.js  
- React (Vite)

---

### Backend Frameworks

- Express (Node.js)
- FastAPI (Python)

---

### Database Support

- Postgres
- MySQL
- MongoDB
- SQLite

Forge generates:
- Database connection code
- Environment variables
- Docker configuration (if applicable)

---

### Authentication Scaffolding

Authentication is optional, but when enabled Forge generates real, working auth code.

Includes:
- JWT-based authentication
- `/login` and `/register` routes
- Token generation and verification
- Middleware / dependencies for request protection

Supported for both Express and FastAPI.

---

### CI/CD Ready

The workflow is tailored to your selected stack and handles:
- Dependency installation
- Build steps
- Basic validation

---

### One-Command Project Setup

Forge generates:

- `frontend/` and `backend/` directories
- `package.json` or `requirements.txt`
- Database configuration
- `.env` and `.env.example`

All in one command.

---

### Docker Support

Forge automatically generates a `docker-compose.yml` file for your selected database, allowing local development without manual setup.

---

## Installation

### Prerequisites

- Rust
- Cargo

---

### Build From Source

```bash
git clone https://github.com/AVBharath10/forge.git
cd forge
cargo install --path .


