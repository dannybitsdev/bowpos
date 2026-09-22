# Architecture Specifications

## Core Principles
- **Domain-Driven Design (DDD)**: Strict separation into Domain, Application, and Infrastructure layers.
- **SOLID & Clean Code**: Single responsibility per handler/component; interfaces over concrete implementations.
- **Data Isolation**: Multitenant isolation enforced via tenant slug headers and PostgreSQL Row-Level Security (RLS).

## Repository Layout
- `/backend`: Rust (Axum + SQLx) REST API running on port 8080 (`/api/` path prefix).
- `/frontend`: React + TypeScript + Vite + Tailwind CSS managed with `pnpm` on port 3000.
- `/infra`: Reverse proxy configs (Nginx unified routing) and Traefik SSL wildcard configuration.