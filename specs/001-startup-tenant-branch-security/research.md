# Research: Secure Startup and Tenant/Branch Authorization

**Feature**: [spec.md](spec.md)  
**Research date**: 2026-10-18  
**Method**: Static inspection of the current repository. No database, migration, build, or test execution was performed.

## Baseline Findings

1. `backend/src/main.rs::main` calls `seed_initial_super_admin` after manually executing every ordered SQL migration. `backend/src/infrastructure/seeder.rs::seed_initial_super_admin` inserts or updates a SUPER_ADMIN with a fixed credential, then calls demo provisioning for two tenants. `ensure_staff_user` also upserts operational accounts with a fixed password. Normal startup therefore has user-creation and credential-reset side effects.
2. The existing first-admin endpoint is exposed through the current auth router at `/register-super-admin`, mounted by `main.rs` under both `/api/auth` and `/api/v1/auth`. `AuthUseCases::register_super_admin` reads `total_users()` and then separately calls `create_tenant` and `create_user`. Static inspection shows no transaction or concurrency serialization around this sequence.
3. `AuthUser::from_request_parts` verifies the JWT, permission policy and token revocation. It constructs the request user from token claims; current role/tenant claims are not reloaded from the user row on each request. Branch IDs are also derived from claims, and the extractor accepts a requested branch when `branch_ids` is empty. `User::requires_explicit_branch()` in `domain/auth.rs` already recognizes `BRANCH_MANAGER`, `CAJERO`, `MESERO` and `COCINERO`, but the extractor's separate `branch.is_none()` role match omits `COCINERO`. `locations::list_locations` also treats an empty assignment list as no branch filter.
4. `x-tenant-id` and `x-tenant-slug` are copied by `tenant_middleware`; protected handlers generally pass `auth.user.tenant_id` to SQL. `X-Tenant-Override` is a separate explicit SUPER_ADMIN path in the extractor. Keep that override distinct from caller-controlled ordinary tenant headers.
5. Auth, location, menu, platform and order handlers use `AuthUser` typed policies. The menu repository binds tenant IDs in CRUD SQL. The order repository sets `app.tenant_id`/`app.branch_scope` in order transactions, but its order catalog is not branch-aware and order creation prices products from the base product record.
6. `orders` is the only table where active RLS was found. Its policy allows access when `app.tenant_id` is absent, and the migration enables but does not force RLS. `main.rs` applies migrations and serves requests through the same database pool; the default Compose database user is also the migration owner. Production role ownership/BYPASSRLS was not verified, so effective RLS enforcement cannot be claimed. Other reviewed tenant-aware SQL operations mainly enforce tenant predicates in application queries.
7. The current migrations declare tenant/business data (`tenants`, `locations`, `users`, `user_branch_access`, `ui_config`, catalog, overrides, orders and order items) and auth-control data (refresh tokens, login attempts, access-token revocations). No bootstrap-state record was found.
8. `deploy.yml` builds and pushes images but does not run test, lint, typecheck or static-analysis jobs first. `frontend/package.json` has lint/test/build scripts but no typecheck script; strict TypeScript mode is already enabled.
9. Git tracks `.env.production` and `frontend/.env.local`. Only names of variables were inspected; no conclusion is drawn about whether any present or historical value is a real secret.

## Decisions

### R1. Separate normal startup from demo provisioning

**Decision**: Normal API startup must not create/reset users or demo data. Make demo provisioning an explicit operation that requires a positively identified non-production environment; missing or invalid environment mode denies demo provisioning. Keep seed routines only where they are needed for non-production setup and tests, and make repeated demo preparation non-destructive to account credentials.

**Rationale**: The current call graph invokes the seeder unconditionally, and the seed functions overwrite password hashes. A separate explicit operation removes the production side effect while preserving the established local/demo workflow.

**Alternatives considered**:
- Keep seeding at startup but skip only the SUPER_ADMIN upsert: rejected because it still creates privileged-adjacent demo users and mixes deployment startup with fixture creation.
- Guard the same startup seeder with a default-on environment flag: rejected because an unset or misconfigured production setting could enable credential creation.
- Remove all demo support: rejected because the spec requires a supported non-production demo path.

### R2. Serialize and persist one-time bootstrap consumption

**Decision**: Keep the existing route/body shape and require an owner-provided, high-entropy `INITIAL_ADMIN_BOOTSTRAP_TOKEN` delivered in the `X-Initial-Admin-Bootstrap-Token` request header. The token is supplied by the deployment secret manager, has no default, is compared without timing-sensitive early exit, and is never logged or persisted. Make authorization, "not previously bootstrapped" checking, tenant/user creation, and consumption of the one-time state one atomic database operation. Use a persistent singleton bootstrap-state record initialized according to whether users already exist. Serialize contenders by locking that state within the same transaction. A failed transaction must not consume the one-time state; a successful one must close the bootstrap permanently.

**Rationale**: A process-local mutex does not coordinate multiple replicas. The current count-then-create sequence is raceable and must not leave a gap between tenant creation, user creation and closure. Persistent state also prevents reopening bootstrap if all users are later deleted.

**Alternatives considered**:
- Check only `COUNT(users) = 0`: rejected because concurrent calls can both pass, and deleting all users can reopen bootstrap.
- Use an in-memory mutex: rejected because it does not coordinate processes or restarts.
- Keep bootstrap as an unauthenticated public route: rejected because request data is not authorization.
- Use a local CLI-only bootstrap: rejected for this feature because the current public operation and concurrent-request acceptance scenarios are HTTP-based; it would create a second provisioning path.
- Create a separate new public route: rejected because the existing bootstrap route can retain its shape and no duplicate endpoint is required.
- Use only a PostgreSQL advisory lock without durable consumed state: rejected because it serializes concurrent work but does not prove that bootstrap was previously consumed if the user table later becomes empty.

**Migration compatibility**: If implementation adds bootstrap state, it must be an additive, idempotent SQL migration compatible with the current startup runner. Existing databases with users must initialize as already bootstrapped. Replacing the migration runner is out of scope.

### R3. Make authenticated branch scope authoritative and fail closed

**Decision**: Reuse the current `AuthUser` and `AccessPolicy` mechanisms. On each protected request, reload the identified user from current server state and reject missing accounts or role/tenant values that no longer agree with the authenticated token; use current server values for scope checks. Treat explicit SUPER_ADMIN override as the only cross-tenant selection and ignore ordinary tenant headers for authorization. Use the existing `requires_explicit_branch()` domain method or an equivalent single source of role truth. For branch-scoped roles, require non-empty current membership and reject unassigned/missing branches regardless of the header. Validate requested branch existence under the effective tenant and confirm assignment from current server-side state, not only stale client claims.

**Rationale**: The existing code already distinguishes branch-scoped roles and carries role and branch claims, but the extractor does not reload current role/tenant and duplicates branch-scope rules incompletely. It also gives empty assignment lists global-looking behavior. Database revalidation prevents stale role, tenant or branch claims from preserving revoked access.

**Alternatives considered**:
- Trust `X-Branch-ID` as membership proof: rejected; it is client-controlled.
- Treat empty branch IDs uniformly as "all branches": rejected for operational roles and incompatible with the spec.
- Trust role, tenant and branch values only from a long-lived token: rejected because the code currently has no per-request user-state check and stale grants would remain valid until token expiry.
- Replace the auth system: rejected as out of scope and unnecessary.

### R4. Use explicit query scope plus transaction-local RLS, with table-specific dispositions

**Decision**: Preserve explicit tenant predicates in SQL and require enforced RLS for every tenant-owned protected table. Apply identity-scoped policies to auth records where those are user/token/email-owned, and define a narrowly scoped policy for non-tenant system control state. Inventory every table and access path, specify how context is established, and prove that the actual runtime database role cannot bypass policies because of table ownership or `BYPASSRLS`. Use forced policies or an appropriately separated non-owner runtime role. For rows requiring request context, set it transaction-locally on the same connection/transaction as protected reads and writes; missing or invalid required context must deny access. Replace the current fail-open `orders` behavior.

**Rationale**: Query predicates exist in many repositories but are easy to omit. Transaction-local settings avoid leaking scope through pooled connections. Applying identical tenant predicates to records with no tenant identity would break legitimate pre-authentication/control flows; those records need identity-scoped or narrowly scoped system policies, not a broad fail-open exception. RLS that is bypassed by the runtime role is not an isolation control.

**Alternatives considered**:
- Enable policies without reviewing login, migration, seeding and the actual runtime role: rejected because a policy that blocks legitimate system operations or is bypassed by its owning role is not an effective control.
- Set session-level tenant state on a pooled connection: rejected because connection reuse can leak one request's context into another.
- Rely solely on application `WHERE tenant_id = ...`: rejected as the only defense due to the Constitution's tenant-first and RLS requirements.
- Replace the SQL migration system or introduce a broad database abstraction: rejected as out of scope.

**Design review gate**: Before implementation is called complete, the table inventory must cover `tenants`, `locations`, `users`, `user_branch_access`, `ui_config`, `categories`, `products`, `modifier_groups`, `modifiers`, `toppings`, `product_modifier_groups`, `product_toppings`, `branch_product_overrides`, `orders`, `order_items`, `order_item_modifiers`, `order_item_toppings`, `auth_refresh_tokens`, `auth_login_attempts`, `auth_access_token_revocations`, and the proposed bootstrap state. Tenant-owned tables require enforced RLS. Authentication lookup before tenant identity is known, tenant-wide platform administration, login attempt tracking, bootstrap, migrations and demo provisioning require narrowly scoped identity/system policies and must not rely on an owner/BYPASSRLS runtime connection.

### R5. Calculate order behavior from branch-effective server data

**Decision**: Extend the existing catalog/order application path to carry the authenticated effective branch. Resolve effective price, stock and availability from the base product plus branch override. At order creation, re-read and validate those values within the order transaction and persist the server-calculated price; reject unavailable or invalid branch products.

**Rationale**: The menu display path already reads branch overrides, but order catalog and creation omit branch scope and use base price. Client-visible pricing cannot be trusted as an order authority.

**Alternatives considered**:
- Leave order creation on base price and treat overrides as display-only: rejected because it can charge the wrong configured price.
- Trust a client-submitted branch or price: rejected because both values are mutable by the caller.
- Build a new pricing module: rejected as unnecessary for the scoped fix; use existing product/override data.

### R6. Remove tracked environment files without disclosing values

**Decision**: Preserve working copies, add ignore coverage, and remove the named files from Git tracking as a separate explicit change. Store the bootstrap token in the deployment secret manager rather than either tracked environment file. Use a reviewer-approved secret scan/review that emits only path, variable name and finding status. If a real current or historical credential is confirmed, refer it to the owner for rotation. Do not rotate automatically, delete local copies, or rewrite history.

**Rationale**: Both paths are currently tracked, but values and history were not inspected. Non-disclosing review is necessary before any claim of exposure or rotation.

**Alternatives considered**:
- Print values for manual review: rejected by the security constraint.
- Delete working copies: rejected because local configuration must be preserved.
- Rewrite Git history automatically: rejected because it is destructive and requires owner coordination.
- Add a new scanning dependency by default: rejected by the Constitution's dependency-approval rule; use an already approved/available tool or obtain explicit approval.

### R7. Make all quality checks required before publication

**Decision**: Add a pull-request validation job using existing Cargo, pnpm, TypeScript, Vitest and PostgreSQL tooling. The image-publish job must depend on successful completion of all mandatory checks. Execute format, Clippy, tests, backend compilation, frontend lint, typecheck, frontend tests and frontend build. Run DB integration tests against a disposable PostgreSQL 16 service.

**Rationale**: The current workflow publishes without quality gates; existing package scripts and dependencies cover most checks, while TypeScript is strict but has no named typecheck script.

**Alternatives considered**:
- Keep validation as optional/manual tasks: rejected because they cannot block image publication.
- Publish and then test the image: rejected because a failed check would be discovered only after release artifacts exist.
- Add a frontend type-checking dependency: rejected; use the TypeScript compiler already installed.
- Run integration tests against development/production data: rejected; tests must use isolated disposable data.

## Phase 1: Design Notes

- The bootstrap state is control-plane data, not tenant business data. Its row is single-instance and must be inaccessible to ordinary tenant requests.
- The owner bootstrap proof is independent of the existing JWT/session contract. It is accepted only while persistent bootstrap state is unused; absence/misconfiguration must deny bootstrap, not fall back to default credentials.
- Existing SUPER_ADMIN tenant override is retained, but branch selection must be validated against that effective tenant. A platform-wide role does not make a nonexistent branch valid.
- Branch-assignment refresh and token claim behavior must be reviewed together: the extractor currently reconstructs a user by claims and the user repository reads branch assignments by user/tenant. A revoked assignment must take effect on the next protected request.
- The `users` table and token revocation checks are used before/while tenant context is established. RLS design must not silently bypass the user's own identity lookup or permit arbitrary cross-tenant account reads.
- `auth_login_attempts` is keyed globally by email and is used before authentication. Its policy should be designed as a narrowly controlled system auth operation, not forced into a tenant predicate with an invented tenant.
- Existing order reads load child items/options after the initial order query. RLS context must remain valid for those reads or their explicit tenant/branch predicates must independently enforce the same scope.
- RLS table coverage, error behavior on missing context, and allowed migration/seeder execution must be included in database integration checks.
- Environment history review and any resulting rotation/remediation are owner-controlled operations, not automatic build steps.
- Exact user-facing HTTP status for unauthorized branch and bootstrap attempts should reuse existing API conventions where possible; the response must not reveal whether another tenant's record exists.

## Phase 1: Design Artifacts

See:

- [data-model.md](data-model.md)
- [contracts/security-behavior.md](contracts/security-behavior.md)
- [quickstart.md](quickstart.md)

These documents define logical state and externally observable behavior. They do not add runtime code or execute a migration.
