# Technology Stack & Quality Standards Specification

## Core Technologies
- **Backend**: Rust 1.80+ (Axum 0.7+, SQLx 0.8+ with PostgreSQL driver, Tokio async runtime).
- **Frontend**: React 18+ (TypeScript 5+, Vite, Tailwind CSS 3.4+).
- **Package Manager**: `pnpm` (strictly enforced; do not use `npm` or `yarn`).
- **State Management**: `zustand` 4+ for client-side global state.
- **Form & Validation**: `react-hook-form` with `zod` schemas.
- **Database**: PostgreSQL 16 with Row-Level Security (RLS).
- **Containerization & Proxy**: Docker Compose, Nginx (unified `/api/` reverse proxy), Traefik v3 (SSL Wildcard Let's Encrypt).

## Linting & Quality Gates
- **Rust**:
  - Compiler linter: `cargo clippy -- -D warnings`
  - Formatter: `cargo fmt --check`
  - Testing: `cargo test`
- **TypeScript / React**:
  - Linter: `pnpm lint` (ESLint with strict TypeScript rules)
  - Type checking: `pnpm typecheck` (`tsc --noEmit`)
  - Testing: `pnpm test` (Vitest + React Testing Library)

## Dependency Rules
- Do NOT introduce new third-party dependencies without explicit approval.
- Maintain pure async I/O in Rust (avoid blocking calls inside Axum handlers).
- Maintain strict TypeScript mode (`"strict": true` in `tsconfig.json`); `any` types are prohibited.