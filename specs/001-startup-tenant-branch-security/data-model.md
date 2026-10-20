# Data Model: Secure Startup and Tenant/Branch Authorization

**Feature**: [spec.md](spec.md)  
**Scope**: Logical entities, identity boundaries and validation rules for this change. Existing database tables are referenced; no migration has been executed.

## Existing Entities

| Entity / table | Identity and relationships | Required isolation / validation |
|---|---|---|
| `tenants` | Tenant identity, name and slug. `users`, locations and business records refer to tenant identity. | A tenant user resolves only to the tenant in the validated identity. Platform listing/override remains restricted to SUPER_ADMIN. |
| `locations` | Belongs to one tenant; branch/location ID is the operational scope. | A requested location must exist in the effective tenant. Operational users must have an active assignment to it. |
| `users` | Belongs to one tenant; has a role and one or more `user_branch_access` rows. | User identity is validated from the signed access token and checked against current server-side state as needed. Tenant and role must not come from caller headers. |
| `user_branch_access` | Connects a user and a location under one tenant; includes primary-branch selection. | User and location must both belong to the same tenant. An empty assignment is no branch access for operational roles, never implicit global access. |
| `ui_config` | Tenant-owned appearance/configuration record. | Reads and writes use the authenticated effective tenant; a tenant header cannot select another tenant's configuration. |
| `categories`, `products` | Tenant-owned catalog; product belongs to a category in the same tenant. | Reads and writes include authenticated tenant scope. Branch-specific catalog display combines base product state with the selected location's override. |
| `modifier_groups`, `modifiers`, `toppings`, `product_modifier_groups`, `product_toppings` | Tenant-owned choices associated to tenant-owned products. | Every association and order selection must be checked against the same tenant and the product being ordered. |
| `branch_product_overrides` | Tenant-owned product/location relation with optional effective price and stock and an availability flag. | Tenant, product and location must agree. The requested branch determines the override; unavailable items cannot be ordered. |
| `orders` | Tenant- and location-owned order, created by a user; has child order items. | Tenant and branch are derived/validated server-side. Existing RLS must fail closed when context is missing. |
| `order_items`, `order_item_modifiers`, `order_item_toppings` | Children of tenant-owned order items and tenant-owned catalog records. | Child fetch and writes must retain the same tenant/branch boundary as the parent order. |
| `auth_refresh_tokens` | Associated with user and tenant; token hash, expiry and revocation lifecycle. | Scope to owning user/tenant; do not expose raw refresh tokens in storage or logs. |
| `auth_login_attempts` | Login control state keyed by email before authentication is established. | Email-identity-scoped auth control; limit each lookup/update to the requested identity and keep responses non-enumerating. |
| `auth_access_token_revocations` | Revocation record keyed by token ID and user ID. | Apply an owning-user/token identity scope; the current table has no tenant ID, so constrain lookups to the authenticated token subject. |

## Proposed Bootstrap State

An additive durable record is required to enforce one-time bootstrap even if users are later deleted. The current migrations contain no equivalent record.

| Logical entity | Proposed attributes | Lifecycle and invariants |
|---|---|---|
| `bootstrap_state` singleton | Stable singleton key; consumed/completed flag; optional completion timestamp. It must not contain the bootstrap secret. | Initialized once by an additive idempotent migration. Existing databases with users initialize as consumed. Bootstrap transaction locks the singleton, verifies unused state and owner authorization, creates tenant and first SUPER_ADMIN, marks consumed, and commits atomically. Failure rolls back both account creation and consumption. |
| Bootstrap authorization proof | Owner-provided high-entropy token delivered in a dedicated request header for the existing first-admin operation; value is not persisted or logged. | Configured as `INITIAL_ADMIN_BOOTSTRAP_TOKEN` through the deployment secret manager, with no default. Compared without timing-sensitive early exit and accepted only while the bootstrap record is unused. Missing, invalid, or unavailable proof denies bootstrap. It does not replace JWT authentication for later operations. |

**Schema decision pending implementation review**: exact singleton key, timestamp requirements and migration details must conform to existing SQL naming/formatting. No generic feature framework or unrelated tables are proposed.

## Request Scope Model

1. **Authenticated actor**: validated access-token subject. On every protected request, confirm the account still exists and its current role and tenant agree with the authenticated claims; reject stale or mismatched identity state. A SUPER_ADMIN may select a different effective tenant only through the existing explicit override mechanism and after server-side validation.
2. **Effective tenant**: actor's validated tenant or an authorized platform override. Ordinary `X-Tenant-ID` and `X-Tenant-Slug` are metadata only and cannot change it.
3. **Effective branch**:
   - Operational roles require a current assignment and an explicitly resolved branch.
   - A requested `X-Branch-ID` is a selector, not proof of access.
   - Branch existence, tenant membership, role scope and user assignment are checked on the server.
   - A missing/empty assignment grants no branch to a branch-scoped actor.
   - Administrative all-branch access is granted only by the existing role/permission model, not by an empty assignment interpreted in isolation.
4. **Database scope**: queries retain explicit tenant/branch predicates. RLS-required operations establish transaction-local scope on the same transaction as the data access. Missing or invalid context denies access.

## Table Isolation Inventory Required Before Completion

The implementation review must classify every item below as (a) tenant/branch RLS protected, (b) tenant/user/token/email-identity RLS protected, or (c) non-tenant system-control data with a narrowly scoped policy and documented access path. Tenant-owned tables must use enforced RLS in the actual runtime role; validate table ownership and `BYPASSRLS`, and use forced policies or a non-owner runtime role. No table may be omitted or left with fail-open missing-context behavior.

| Table | Expected classification to evaluate |
|---|---|
| `tenants` | Tenant-scoped; explicit SUPER_ADMIN platform list/override path. |
| `locations` | Tenant + branch access scope. |
| `users` | Tenant-scoped user/admin operations; pre-auth lookup is limited to an identity-scoped authentication operation. |
| `user_branch_access` | Tenant + target user/branch relationship. |
| `ui_config` | Tenant-scoped configuration. |
| `categories`, `products` | Tenant-scoped catalog; product reads may additionally vary by branch. |
| `modifier_groups`, `modifiers`, `toppings`, `product_modifier_groups`, `product_toppings` | Tenant-scoped catalog relations. |
| `branch_product_overrides` | Tenant + location-scoped. |
| `orders` | Tenant + branch-scoped; replace current missing-context fail-open. |
| `order_items`, `order_item_modifiers`, `order_item_toppings` | Parent order tenant/branch scope must carry through. |
| `auth_refresh_tokens` | Owning user + tenant scope. |
| `auth_login_attempts` | Email-identity-scoped auth control; access is needed before identity validation and must remain limited to the requested identity. |
| `auth_access_token_revocations` | Owning user/token RLS scope; current table has no tenant ID and must use the authenticated token subject to constrain lookup. |
| Bootstrap state | Non-tenant system control plane; only bootstrap and migration operations access the singleton through a narrow policy. |

## Security State Transitions

### Bootstrap

`Unconsumed + valid owner proof + empty user set` → atomic creation of initial tenant and SUPER_ADMIN → `Consumed`.

- Any missing/invalid proof, pre-existing user, consumed state, failed tenant/user insert, or failed state update results in no successful bootstrap.
- Contenders serialize on persistent database state; exactly one successful transaction can transition to `Consumed`.
- A retry after a committed success is rejected. A retry after a rolled-back failure may retry if the bootstrap remains unconsumed.
- Deleting users later does not reset the state.

### Branch request

`Validated identity` → determine effective tenant → resolve requested/implicit branch → validate tenant membership and current role/assignment → authorized data access.

- Any invalid or missing required assignment denies access.
- A client-controlled header cannot transition an actor into a broader role, tenant or branch scope.
- Database lookup errors fail closed.
