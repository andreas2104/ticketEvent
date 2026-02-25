# TicketApp Monorepo

Welcome to TicketApp. This repository contains both the backend and frontend for the ticket booking system.

## Project Structure

- `backend/`: Rust application (Axum + SQLx + SQLite)
- `frontend/`: Next.js application (TypeScript + Tailwind CSS)

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js & pnpm
- Docker & Docker Compose (optional, for local orchestration)

### Development

#### Run Backend
```bash
cd backend
cargo run
```

#### Run Frontend
```bash
cd frontend
pnpm dev
```

### Docker Compose
You can also run the entire stack using Docker Compose:
```bash
docker-compose up --build
```
# ticketEvent
