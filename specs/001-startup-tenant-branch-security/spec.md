# Feature Specification: Secure Startup and Tenant/Branch Authorization

**Feature Branch**: `not created (no before_specify hook is configured)`

**Created**: 2026-10-18

**Status**: Draft

**Input**: User description: Harden backend startup, tenant/branch authorization, environment-file handling, and quality gates for the existing BowPOS application without rebuilding existing functionality.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start and bootstrap safely (Priority: P1)

As the system owner, I need production startup to preserve existing accounts and an authorized first administrator to be created only once, so that a restart or simultaneous bootstrap requests cannot restore or create default privileged credentials.

**Why this priority**: An exposed privileged account or race during initial setup can compromise every tenant.

**Independent Test**: Start with an empty database in production mode and with an existing administrator; repeat startup and submit concurrent first-administrator requests. No default account is created or reset, and at most one authorized bootstrap succeeds.

**Acceptance Scenarios**:

1. **Given** a production environment with no users, **When** the backend starts, **Then** it does not create demo users or a privileged account with predetermined credentials.
2. **Given** an existing privileged account, **When** the backend restarts, **Then** its credentials and role are not reset by startup.
3. **Given** an empty system and an authorized one-time bootstrap, **When** concurrent requests attempt to create the first privileged administrator, **Then** exactly one request can create it and all later or competing attempts are rejected without creating additional tenants or administrators.
4. **Given** a system with any existing user, **When** an unauthenticated or repeated first-administrator registration is attempted, **Then** the request is denied and persisted account data is unchanged.
5. **Given** an explicitly selected non-production demo setup, **When** it is prepared, **Then** demo data is available without resetting credentials for existing accounts; production rejects demo setup.

### User Story 2 - Operate only within assigned tenant and branches (Priority: P1)

As an employee or administrator, I need every protected operation to respect my authenticated identity, role, permissions, tenant, and authorized branch scope, so that changing request headers or resource identifiers cannot grant access to another restaurant or location.

**Why this priority**: Cross-tenant or cross-branch access exposes business and customer data and permits unauthorized operational changes.

**Independent Test**: Use at least two tenants and multiple branches with users assigned to different scopes. Exercise reads and writes for locations, catalog, overrides, and orders using valid and tampered identities, headers, and IDs.

**Acceptance Scenarios**:

1. **Given** an operational user with no branch assignments, **When** they request any branch, including by supplying `X-Branch-ID`, **Then** the request is denied and returns no branch data.
2. **Given** an operational user assigned to one branch, **When** they request an unassigned branch or another tenant's resource by changing a header or identifier, **Then** the request is denied and no data is disclosed or changed.
3. **Given** an authenticated tenant user, **When** they change ordinary tenant ID or slug headers, **Then** the effective tenant remains the tenant validated for their identity.
4. **Given** a platform administrator using the existing explicit tenant-override capability, **When** the target tenant is valid and the user's permissions authorize the operation, **Then** only that authorized scope is available; invalid targets and unauthorized roles are denied.
5. **Given** an administrator with permissions for multiple branches and an operational user assigned to one branch, **When** each performs an otherwise authorized action, **Then** the administrator retains only the scope granted by existing permissions and the operational user remains limited to their assignment.
6. **Given** an order or catalog request, **When** the selected branch has an override for price, stock, or availability, **Then** catalog results and order validation use that branch's effective values; a product unavailable at that branch cannot be ordered there.
7. **Given** a protected data request with missing or invalid tenant context, **When** it reaches any in-scope data access path, **Then** access is denied rather than falling back to an unrestricted scope.

### User Story 3 - Protect environment configuration (Priority: P2)

As the repository owner, I need environment files and secrets kept out of version control and review artifacts, so that configuration can be checked and corrected without disclosing credentials or rewriting history without approval.

**Why this priority**: Tracked production configuration can expose database and authentication secrets; investigation and remediation must not create further disclosure.

**Independent Test**: Confirm the production and local environment files are no longer tracked while local copies are preserved, and review the current contents and repository history using a process that reports only secret names and findings, never values.

**Acceptance Scenarios**:

1. **Given** `.env.production` and `frontend/.env.local` currently tracked by Git, **When** this feature is completed, **Then** they are no longer tracked, local copies are not automatically deleted, and no secret values appear in the specification, logs, or review output.
2. **Given** the controlled review finds a real credential in a current file or repository history, **When** the finding is reported, **Then** it identifies the variable/file and recommends owner-approved rotation without printing the value or rotating it automatically.
3. **Given** history rewriting or file deletion is proposed, **When** explicit owner approval has not been given, **Then** neither action is performed.

### User Story 4 - Block releases that fail quality checks (Priority: P2)

As the release owner, I need the required backend and frontend quality checks to pass before container images are published, so that insecure or broken changes cannot bypass review through the deployment workflow.

**Why this priority**: The current publishing workflow does not enforce the documented quality gates before publishing images.

**Independent Test**: Cause each required check to fail in a pull request or controlled workflow run and verify publishing is blocked; then verify a successful run executes the complete required check set before publishing.

**Acceptance Scenarios**:

1. **Given** a change fails any required formatting, lint, static-analysis, type-check, test, or build check, **When** publication is requested, **Then** no image is published.
2. **Given** a change passes all required checks, including database-backed security integration tests, **When** publication is requested, **Then** publication occurs only after the checks complete successfully.
3. **Given** a security change touches tenant or branch authorization, **When** its checks run, **Then** integration coverage includes at least two tenants, multiple branches, authorized actors, and unauthorized cross-scope attempts.

### Edge Cases

- Startup receives an invalid or contradictory environment mode; it must fail safely without creating users or demo data.
- A bootstrap request is interrupted after the first administrator is created; retries must not create another privileged administrator or tenant.
- Multiple bootstrap attempts use different emails or tenants simultaneously; the one-time rule still holds.
- An operational user has zero assignments, a deleted assignment, or an assignment to a branch from another tenant; each case denies access.
- A privileged user supplies both a tenant override and a branch ID that does not belong to that tenant; the request is denied.
- A database/context lookup fails while validating tenant or branch membership; the system must not interpret the failure as unrestricted access.
- Branch override values are absent, explicitly unavailable, or have zero stock; order behavior must respect the effective branch configuration.
- A required integration test cannot connect to its isolated database; the quality gate fails and publication is blocked.
- Secret review finds no current values but cannot establish whether historical values were valid; the result is reported as unresolved and no automatic history rewrite occurs.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Normal production startup MUST NOT create demo tenants, demo users, or privileged accounts with predetermined credentials.
- **FR-002**: Startup MUST NOT reset credentials, roles, or tenant assignments of existing users as a side effect of preparing demo or initial data.
- **FR-003**: Demo data preparation MUST be an explicit non-production action; production MUST refuse the demo-data mode.
- **FR-004**: Initial privileged-account creation MUST require authorization by the deployment owner, MUST be usable only once, and MUST remain safe when attempts are concurrent or retried.
- **FR-005**: Once any user or initial administrator exists, the public first-administrator flow MUST NOT create another initial privileged account. Existing authenticated account-creation flows and their role restrictions MUST remain intact.
- **FR-006**: Every protected operation MUST derive the effective user, role, permission set, and ordinary tenant scope from a validated authenticated identity; caller-controlled tenant identifiers or slugs MUST NOT change that scope.
- **FR-007**: An explicit platform tenant override, if used, MUST remain restricted to the existing authorized platform role, a validated target tenant, and the permissions applicable to the requested operation.
- **FR-008**: Users with operational, branch-scoped roles MUST have a valid assignment to the requested branch. An empty, invalid, revoked, or cross-tenant assignment MUST grant no branch access, regardless of `X-Branch-ID`.
- **FR-009**: Administrative access across branches MUST be limited to the scope granted by the existing role and permissions; branch-scoped administrators MUST NOT gain global scope from a missing assignment or caller-supplied branch.
- **FR-010**: Tenant and branch checks MUST be applied consistently to protected reads and writes for locations, catalog products and modifiers, branch overrides, orders, and order details. Resource identifiers from another tenant or unauthorized branch MUST not reveal or alter data.
- **FR-011**: Catalog display and order creation MUST use the effective branch's configured price, stock, and availability when an override exists, and MUST reject unavailable products for that branch.
- **FR-012**: Before tenant isolation is declared complete, the implementation MUST inventory tenant-aware tables and data-access operations, assess which require database-level row policies as defense in depth, document the decision for each, and ensure protected access fails closed when required tenant context is absent or invalid. This assessment MUST include tenant, branch, user-assignment, catalog, override, order, and order-item data; it MUST also account for authentication records that contain tenant or user identifiers.
- **FR-013**: `.env.production` and `frontend/.env.local` MUST cease to be tracked by Git while any local copies are preserved. Review outputs MUST identify only paths, variable names, and finding status, never values.
- **FR-014**: Current environment-file contents and repository history MUST be reviewed for actual secrets using an approved non-disclosing process. Confirmed credentials MUST be referred for owner-approved rotation; no credential rotation, file deletion, or history rewrite occurs automatically.
- **FR-015**: The required backend and frontend format, lint, static-analysis, type-check, test, and build checks MUST run before image publication. Any failed or skipped mandatory check MUST block publication.
- **FR-016**: Database-backed integration tests MUST run against an isolated test database and cover at least two tenants and multiple branches, including authorized and unauthorized requests for the in-scope resources.
- **FR-017**: The change MUST preserve existing authentication, account-management, location, catalog, override, and order contracts except where behavior is explicitly tightened by these requirements. It MUST NOT add duplicate routes or replace the existing authentication/session model.
- **FR-018**: No new third-party dependency may be introduced without explicit approval. The work MUST NOT add inventory, purchasing, payment processing, electronic invoicing, QR customer ordering, or AI functionality.

### Existing Context and Compatibility

These facts were revalidated against the current repository before drafting this specification. They describe the baseline, not a prescribed implementation:

- The backend calls `seed_initial_super_admin` at startup. [`main.rs`](../../backend/src/main.rs) invokes [`seed_initial_super_admin`](../../backend/src/infrastructure/seeder.rs), which upserts a privileged account and demo operational users using fixed credentials. `seed_tenant_full` also prepares demo tenants, branches, catalog, and orders.
- The existing first-admin route is `/api/auth/register-super-admin` and is also mounted under `/api/v1/auth`. [`auth_router`](../../backend/src/presentation/auth/router.rs) exposes it; [`register_super_admin`](../../backend/src/application/auth/use_cases.rs) checks `total_users() > 0` before separately creating a tenant and user. Static inspection shows no transaction/serialization around that sequence.
- [`AuthUser::from_request_parts`](../../backend/src/presentation/auth/extractor.rs) validates JWT, role/permission policy, tenant claims, and branch context. It currently accepts any requested branch when `branch_ids` is empty; its branch-required role list does not include `COCINERO`.
- Existing protected data surfaces include [`locations_router`](../../backend/src/handlers/locations.rs), [`menu_router`](../../backend/src/handlers/menu.rs), [`orders_router`](../../backend/src/handlers/orders.rs), and [`platform_router`](../../backend/src/handlers/platform.rs). The `X-Tenant-Override` behavior for `SUPER_ADMIN` is part of the existing platform flow and must not become a general tenant-header override.
- Tenant-aware tables visible in the migrations include `tenants`, `locations`, `users`, `user_branch_access`, `ui_config`, `categories`, `products`, `modifier_groups`, `modifiers`, `toppings`, `product_modifier_groups`, `product_toppings`, `branch_product_overrides`, `orders`, `order_items`, `order_item_modifiers`, and `order_item_toppings`. Authentication tables include refresh tokens, login attempts, and access-token revocations.
- Static search found active RLS only on `orders`, with a policy that permits access when `app.tenant_id` is absent. [`apply_rls_scope`](../../backend/src/infrastructure/repositories/sqlx_orders_repository.rs) sets tenant and branch context for order transactions. Other reviewed repository/handler queries generally bind a tenant identifier, but a full query-by-query isolation review remains required.
- [`SqlxUserRepository::list_menu`](../../backend/src/infrastructure/repositories/sqlx_user_repository.rs) reads branch overrides. The order catalog and order creation in [`SqlxOrderRepository`](../../backend/src/infrastructure/repositories/sqlx_orders_repository.rs) do not take branch context for the catalog and use the base product price during order creation.
- Git currently tracks `.env.production` and `frontend/.env.local`. The production file declares sensitive-looking variable names including `DATABASE_URL`, `JWT_SECRET`, `REFRESH_TOKEN_SECRET`, and `POSTGRES_PASSWORD`; the frontend file declares `VITE_API_URL`. Names were inspected without reading or displaying values; whether current or historical values are real secrets is not yet established.
- The only workflow found is [`deploy.yml`](../../.github/workflows/deploy.yml), which builds and publishes images without required quality jobs. [`frontend/package.json`](../../frontend/package.json) defines lint, test, and build scripts; it has no `typecheck` script. Static inspection is not proof of test or build status; no checks were run for this specification.

### Key Entities *(include if feature involves data)*

- **Authenticated identity**: The validated account, role, permissions, and tenant scope used to authorize a protected request.
- **Tenant**: A restaurant organization whose data must remain isolated from other organizations.
- **Branch assignment**: The relationship granting a user access to one or more locations within their tenant.
- **Protected business record**: Tenant- or branch-owned data such as a location, catalog item, override, order, or order detail.
- **Bootstrap authorization**: The deployment-owner authorization that permits initial privileged-account creation once.
- **Quality-gate result**: The outcome of mandatory validation checks that determines whether a release may be published.
- **Environment secret finding**: A non-disclosing report identifying a file or variable name and whether a real secret was confirmed, without including its value.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Across fresh production startup and at least 10 subsequent restarts, zero default privileged accounts or demo users are created, and zero existing credentials or roles are reset by startup.
- **SC-002**: Across 20 concurrent attempts to bootstrap an empty system, exactly one authorized attempt creates the initial privileged administrator; all other attempts make no additional privileged accounts or tenants.
- **SC-003**: In an integration matrix with at least two tenants and two branches per tenant, 100% of requests using an invalid identity, unauthorized tenant, unassigned branch, or tampered identifier/header are denied without data disclosure or mutation.
- **SC-004**: All documented valid administrative and branch-assigned user scenarios continue to succeed under their existing permissions.
- **SC-005**: Every tenant-aware table and protected data operation in the reviewed scope has a recorded isolation decision; no protected operation succeeds when its required tenant or branch context is missing or invalid.
- **SC-006**: The two named environment files are absent from Git tracking after remediation, and 0 secret values appear in repository review artifacts, logs, test output, or the specification.
- **SC-007**: 100% of mandatory backend and frontend quality checks complete successfully before publication; a single failing or skipped mandatory check results in zero published images.

## Assumptions

- The existing JWT authentication, RBAC policies, user-branch assignments, endpoint contracts, and platform override flow are reused where they meet the requirements.
- Operational roles, including `BRANCH_MANAGER`, `CAJERO`, `MESERO`, and `COCINERO`, are branch-scoped unless an existing explicit permission grants a broader scope.
- The deployment owner can authorize a one-time initial bootstrap. The exact proof and operational delivery procedure are implementation decisions for the plan; it must not be an unauthenticated, repeatable public action.
- Demo data is needed for development or testing, but not for normal production startup.
- Environment contents and history have not yet been validated for actual secrets. This specification does not authorize disclosure, automatic rotation, deletion of local files, or history rewriting.
- Existing SQL migrations may be extended only as needed for these security requirements; replacing the full migration system is out of scope.
- A full tenant-isolation review may identify additional tables or query paths beyond the currently inspected list; the implementation must extend its inventory rather than treating this list as exhaustive.
- Authentication/session migration, inventory, purchasing, payments, electronic invoicing, QR ordering, and AI remain separate future decisions and are not prerequisites for this feature.
