# Implementation Plan: Secure Startup and Tenant/Branch Authorization

**Branch**: `001-startup-tenant-branch-security` (feature identifier; no Git branch was created) | **Date**: 2026-10-18 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-startup-tenant-branch-security/spec.md`

## Summary

Harden the existing BowPOS startup and protected request paths without replacing the current Rust/Axum/PostgreSQL or React/TypeScript stack. Remove privileged/demo account seeding from normal startup, retain demo provisioning as an explicit non-production operation, and make the existing first-admin route a deployment-owner-authorized, durable one-time bootstrap serialized in PostgreSQL. Revalidate current identity, role, tenant and branch scope on protected requests. Audit every tenant-owned SQL path, enforce fail-closed RLS and explicit query scope, and document narrow policies for auth/system-control records that have no tenant owner. Verify the actual runtime database role cannot bypass those policies. Preserve the current endpoint shapes except for the authorization requirement on the existing bootstrap route. Stop tracking the two named environment files without deleting local copies, review secrets without disclosure, and make quality checks a prerequisite for image publication.

The research and design artifacts below record the current repository evidence and constraints. They are planning decisions, not implementation results; no tests or migrations have been run as part of this plan.

## Technical Context

**Language/Version**: Rust 2021 edition; production/development Docker stages use Rust 1.88. Frontend uses TypeScript 5.6 with strict mode.

**Primary Dependencies**: Axum 0.7, SQLx 0.8 with PostgreSQL, Tokio, existing JWT/password-hashing libraries; React 18, Vite, ESLint, Vitest and pnpm 9.15. No new third-party dependency is planned.

**Storage**: PostgreSQL 16. Existing schema and SQL migrations are under `backend/migrations`; the backend applies ordered SQL files on startup with `sqlx::raw_sql`.

**Testing**: Rust unit tests use `cargo test`; database-backed integration tests will use an isolated PostgreSQL 16 service. Frontend checks use existing `pnpm lint`, `pnpm test`, and `pnpm build` scripts. Add a `typecheck` script that invokes the already-installed TypeScript compiler for the strict application configuration and the Vite config under strict mode; no dependency is needed.

**Target Platform**: Linux containers (Docker/Render deployment) and local development on Docker Compose.

**Project Type**: Existing web application with Rust API backend and React SPA frontend.

**Performance Goals**: Preserve current endpoint behavior and normal request usability. Branch membership and RLS context validation must not introduce unbounded queries; no new numeric latency target is specified.

**Constraints**: Preserve the current auth/session scheme and route contracts, except for enforcing authorization on the existing first-admin bootstrap. Do not publish secret values, delete local environment files, rotate credentials, rewrite Git history, execute destructive migrations, add dependencies without approval, or implement excluded business modules. Any new SQL migration must be additive and safe under the current repeated-startup migration runner.

**Scale/Scope**: Existing backend auth/bootstrap, protected tenant/branch data access for locations, catalog, overrides, orders and related records, environment-file tracking/review, and CI image-publication gating. RLS scope decisions must cover all tenant-aware/auth-control tables discovered in the current migrations.

## Constitution Check

| Gate | Status | Evidence / plan response |
|---|---|---|
| Tenant-first boundaries use an authenticated tenant and branch scope | PASS, subject to implementation proof | The protected request must revalidate the identity's current role/tenant and explicit SUPER_ADMIN override; ordinary tenant headers are not authoritative. Every tenant-owned table needs enforced RLS and explicit query scope; system/auth-control policies need documented narrow identities. |
| Protected operations use default-deny authentication and explicit policy | PASS, with baseline defect tracked | Existing typed `AuthUser<P>` policies are reused. Branch authorization must deny operational actors with empty or stale assignments, including `COCINERO`. |
| Domain/application/infrastructure responsibilities remain explicit | PASS | Reuse existing application use cases and repositories; make multi-row bootstrap atomic at repository boundary. No broad architecture rewrite is planned. |
| Quality gates are executed before merge/publication | PASS as target; baseline gap documented | Required Rust and frontend checks become CI gates. Image publishing depends on successful checks; this plan does not claim any check has run or passed. |
| Approved stack, async I/O, no unapproved dependencies | PASS | Existing stack and PostgreSQL are sufficient. RLS, transactions and repository tests use existing SQLx/PostgreSQL capabilities. |

**Pre-design gate**: Pass. The planned additive bootstrap state and targeted policy changes are directly justified by the one-time bootstrap and fail-closed isolation requirements; they do not replace the migration runner or session model.

**Post-design gate**: Pass with an explicit implementation review gate. Table-specific policy behavior for pre-authentication/system records must be reviewed before adding RLS. No data surface may be called fully isolated until its application filter and database policy/justified system-scope decision have been verified.

## Phase 0: Research

Completed static research is recorded in [research.md](research.md). Decisions that guide planning:

- Do not seed demo or privileged users during ordinary service startup. Keep provisioning explicit and reject demo provisioning in production.
- Preserve the existing first-admin route contract, but require an out-of-band, owner-provided bootstrap authorization and enforce single-use inside a database transaction protected by persistent bootstrap state. A process-local mutex or unprotected user count is not sufficient across replicas/restarts.
- Treat branch assignments as server-authoritative. Revalidate requested branch membership against the effective tenant and authenticated role; an empty list is not global access for operational roles.
- Carry tenant/branch context through the same transaction as protected SQL where row policies apply; do not use pooled connection session variables that can leak context between requests.
- Inventory every tenant-aware table and classify system/auth-control tables separately. Enforce database row policies on every tenant-owned table; give non-tenant auth/control records narrowly scoped identity/system policies. Missing required context must deny protected access, and the actual runtime role must not bypass policy enforcement. Keep explicit tenant predicates in queries as a second control.
- Use branch-effective catalog pricing and availability both when returning a branch catalog and when creating an order. Re-read authoritative values during order creation rather than trusting client state.
- Preserve local environment files while removing them from tracking; perform current/history secret review using a non-disclosing, owner-approved process. No automated rotation or history rewrite.
- Add a validation job before image publishing. Use existing commands/tools only; frontend typecheck can call the existing TypeScript compiler without adding a dependency.

## Phase 1: Design & Contracts

The design artifacts are:

- [data-model.md](data-model.md): existing entities and proposed bootstrap lifecycle/scope invariants.
- [contracts/security-behavior.md](contracts/security-behavior.md): protected route contract and required authorization behavior without introducing routes.
- [quickstart.md](quickstart.md): disposable-database validation path and CI check sequence.

No customer-facing endpoint or response shape is intended to change. The existing `/api/auth/register-super-admin` and `/api/v1/auth/register-super-admin` aliases remain the bootstrap surface, with an additional required owner authorization proof and one-time semantics.

## Project Structure

### Documentation (this feature)

```text
specs/001-startup-tenant-branch-security/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── security-behavior.md
└── tasks.md                 # Produced later by /speckit-tasks
```

### Source Code (repository root)

```text
backend/
├── migrations/              # Additive bootstrap-state / RLS policy SQL if approved by plan
└── src/
    ├── application/auth/    # Bootstrap policy and account use cases
    ├── domain/              # Existing role and user branch-scope rules
    ├── handlers/            # Existing locations, menu and orders endpoints
    ├── infrastructure/
    │   ├── repositories/    # Atomic bootstrap and tenant/branch-scoped SQL
    │   └── seeder.rs        # Explicit non-production demo provisioning
    ├── middleware/          # Existing tenant-context transport (not authority)
    └── presentation/auth/   # Existing auth route, policy and AuthUser extractor
frontend/
└── package.json             # Existing lint/test/build plus explicit typecheck script
.github/
└── workflows/
    └── deploy.yml           # Validation gate must succeed before image publication
.gitignore                   # Ignore protected local env files
specs/                       # Feature documentation only
```

**Structure Decision**: Extend the existing backend vertical slices and repository, keep the current public routes, add only the smallest additive SQL state/policy changes justified by the design, and gate the existing image-publication workflow. The frontend has no planned feature UI changes. Local environment file copies remain outside Git tracking; their values are not documented.

## Delivery Sequence and Verification Gates

1. **Secure startup and bootstrap**: remove seeder calls from ordinary startup; make demo provisioning explicit/non-production; add durable single-use bootstrap state and an atomic owner-authorized use case on the existing route. Validate empty/existing databases, retries, concurrent requests, and preservation of existing accounts.
2. **Server-authoritative tenant and branch scope**: fix empty-assignment semantics; validate branch existence/tenant/assignment on protected operations and maintain the platform override boundary. Add branch context to order catalog/creation and use effective price/availability. Test operational, administrative and SUPER_ADMIN paths.
3. **Tenant data isolation review**: inventory SQL tables/operations and document RLS disposition for business, auth and system-control data. Close fail-open protected policies, use transaction-local context, prove the runtime DB role cannot bypass RLS, and test cross-tenant/cross-branch attempts. Do not mark full isolation complete until every in-scope query is accounted for.
4. **Environment-file handling**: add ignore rules and stop tracking the named files without deleting working copies. Review current content and history with output redaction, report only variable names/status, and leave rotation/history rewrite to explicit owner approval.
5. **Quality workflow**: run required Rust/frontend controls on pull requests; publish only after all mandatory jobs succeed. Provision PostgreSQL as an isolated CI service for integration tests; never point tests at production data.

Mandatory pre-publication checks, run in a separate validation job:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test` with PostgreSQL service configuration for database-backed integration tests
4. `cargo build --locked`
5. `pnpm lint`
6. `pnpm typecheck` (script checks the app and Vite TypeScript configurations under strict mode using the existing compiler)
7. `pnpm test`
8. `pnpm build`

Image publishing must depend on successful completion of the mandatory validation job. Any failed or skipped required check blocks image publication. Checks and integration tests are proposed for the future implementation; none were executed for this plan.
