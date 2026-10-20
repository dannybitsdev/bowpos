# BowPOS Constitution
<!-- Sync Impact Report
- Version change: n/a → 1.0.0
- Modified principles: n/a → initial principle set
- Added sections: Core Principles, Technology Standards, Development Workflow
- Removed sections: none
- Follow-up TODOs: none
-->

## Core Principles

### I. Tenant-First Architecture
BowPOS MUST treat tenant isolation as a non-negotiable design constraint. Every route, repository, and data access boundary MUST enforce tenant context through explicit tenant identifiers and PostgreSQL Row-Level Security. Shared infrastructure is permitted only when tenant boundaries are enforced at the database and application layer.

Rationale: BowPOS serves multiple restaurant tenants and location groups; failure to isolate data creates direct confidentiality and integrity risk.

### II. Security and Authorization by Default
BowPOS MUST use authenticated, permission-checked access for every protected action. JWT-based flows, HTTP-only session patterns, and explicit RBAC checks MUST be adopted for all handlers, with default-deny access unless a route declares required permissions.

Rationale: Authorization defects are the highest-risk operational issue in a POS system because they expose orders, payments, and tenant data.

### III. Domain-Driven Design and Clear Boundaries
BowPOS MUST separate domain logic, application use cases, and infrastructure dependencies. Repositories, handlers, services, and value objects MUST keep responsibilities explicit; cross-layer shortcuts are prohibited when they blur business rules or data ownership.

Rationale: The domain has strict business rules around orders, menu management, tenants, and branch scoping; clear architecture reduces regressions.

### IV. Quality Gates and Testability
BowPOS MUST keep code reviewable and verifiable through automated quality gates. Rust code MUST pass cargo fmt and cargo clippy with warnings treated as errors; TypeScript code MUST pass strict type checking and linting; tests MUST run for changed behavior before merge.

Rationale: A POS platform must remain reliable under rapid menu, order, and tenant changes, especially where operational mistakes affect real business transactions.

### V. Technology Consistency and Simplicity
BowPOS MUST stay within the approved stack: Rust 1.80+ with Axum, SQLx, PostgreSQL, React 18 with TypeScript, Vite, Tailwind, Zustand, react-hook-form, and zod. New dependencies MUST be justified and approved before introduction, and solutions MUST prefer simple, explicit contracts over broad abstractions.

Rationale: A constrained stack reduces hidden coupling, deployment risk, and maintenance cost.

## Technology Standards
This project standardizes on:
- Backend: Rust 1.80+ on Tokio with Axum and SQLx using PostgreSQL 16 and RLS.
- Frontend: React 18 + TypeScript 5 + Vite + Tailwind CSS 3.4+, with Zustand and form validation via react-hook-form + zod.
- Package manager: pnpm; npm and yarn are not used for this project.
- Proxy and deployment: Docker Compose, Nginx unified `/api/` routing, and Traefik v3 with wildcard TLS where required.
- Async discipline: Rust handlers MUST remain non-blocking and I/O-bound operations MUST remain async.

## Development Workflow
- Features MUST be implemented in a way that preserves the approved architecture and technology constraints.
- Changes MUST be verified through the relevant project checks before merge: cargo fmt, cargo clippy, cargo test, pnpm lint, pnpm typecheck, and pnpm test where applicable.
- All security-sensitive changes, RBAC updates, and tenant-scoping changes MUST be reviewed for data isolation and authorization correctness.
- Branches, menu structures, and tenant-aware flows MUST be explicitly scoped and documented when they affect business context or user permissions.
- Deployment changes MUST preserve the repository's reverse-proxy and container model and must not introduce ad hoc environment assumptions.

## Governance
This Constitution supersedes conflicting local conventions or one-off practices. Amendments require project-level documentation, explicit rationale, and review against the standards in this document before adoption.

Amendments MUST:
- document the change in the project constitution,
- explain why the prior rule was insufficient or incompatible,
- identify any migration, rollout, or compliance impact,
- and keep the version number aligned with semantic versioning rules.

Versioning policy:
- MAJOR: breaking governance or principle changes,
- MINOR: new principles or materially expanded guidance,
- PATCH: clarification or non-semantic wording changes.

Compliance review:
- Any pull request affecting architecture, security, data access, or deployment MUST verify compliance with this Constitution.
- Non-compliance MUST be resolved before merge, unless an explicit exception is documented and accepted by the project owner.

**Version**: 1.0.0 | **Ratified**: 2026-10-13 | **Last Amended**: 2026-10-13
