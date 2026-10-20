# Validation Quickstart

**Feature**: [spec.md](spec.md)  
**Status**: Planned validation guide. Commands have not been executed.

## Safety Preconditions

- Use a disposable PostgreSQL 16 database. Never point integration tests or seed commands at production or a shared development database.
- Do not copy production environment values into test output or documentation. Supply test-only connection strings and signing secrets through the local/CI secret mechanism.
- Do not run migrations against production as part of this guide. Any migration validation must target a disposable test database.
- Demo provisioning is permitted only in an explicitly non-production environment.

## Backend Unit and Static Checks

From `backend\`:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --locked
```

Expected outcome: every command exits successfully. Database-backed authorization tests run only with the isolated test database configured; tests must fail rather than silently skip if the required CI database service is unavailable. Existing explicitly ignored tests may remain ignored only if their scenarios are covered by the required integration suite.

## Database-Backed Security Integration Scenarios

Use a clean, isolated PostgreSQL 16 database and the approved test fixture mechanism. Apply the existing additive migrations only to that disposable database. Run the integration suite that covers:

1. Two tenants with at least two branches each.
2. SUPER_ADMIN, tenant administrator, branch manager, cashier, waiter and cook identities with zero, one and multiple assignments as applicable.
3. Valid bootstrap authorization, missing/invalid proof, sequential repeat, concurrent attempts, rollback/retry, and startup restart.
4. Ordinary tenant headers tampered against JWT tenant, explicit authorized platform override, invalid override, invalid branch under override, and cross-tenant resource IDs.
5. Branch reads/writes for locations, product catalog, modifiers, overrides, orders, order details and status updates.
6. Deleted users and stale role/tenant claims; reject them on protected requests. Missing, revoked and stale branch assignments must also be rejected; operational users with no assignments are denied even when they supply a branch header.
7. RLS and explicit query filters with valid, absent and mismatched tenant/branch context. Verify policies under the actual application runtime role, including table ownership and `BYPASSRLS`; missing context must deny protected data access.
8. Branch override price/stock/availability behavior for both catalog output and server-side order total; unavailable product creation must fail.

Expected outcome: unauthorized requests are denied without cross-scope disclosure or mutation; authorized cases continue to work. The test suite must verify persisted state as well as HTTP status.

## Frontend Checks

From `frontend\`:

```powershell
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```

Expected outcome: all checks exit successfully. The `typecheck` script must check both the app and Vite TypeScript configurations under strict mode using the existing compiler, rather than weakening compiler settings.

## CI Publication Gate

The pull-request validation workflow must run the Rust commands and frontend commands above, with a disposable PostgreSQL 16 service for integration tests. The existing image publication job must depend on the validation job.

Expected outcomes:

- Deliberately failing each mandatory job blocks image publication.
- Successful validation permits publication only after all required checks finish.
- No job emits environment values or test secrets.

## Environment File Review

1. Preserve local working copies of `.env.production` and `frontend/.env.local`.
2. Confirm the paths are no longer tracked after the owner-approved repository change.
3. Review current content and history using a non-disclosing approved method; output only path, variable name, and finding status.
4. If a real secret is confirmed, notify the owner for rotation through the normal secret-management process. Do not print values, rotate automatically, delete local copies, or rewrite history from this guide.
