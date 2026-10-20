# Security Behavior Contract

**Feature**: [spec.md](../spec.md)  
**Purpose**: Specify authorization semantics for existing external routes. This document does not add routes or prescribe a response schema change.

## Compatibility Boundary

- Preserve current route paths, request payloads, and successful response shapes except for requiring owner authorization on first-admin bootstrap and denying requests outside the actor's scope.
- Preserve the existing JWT Bearer authentication and refresh/logout contracts. Cookie migration is out of scope.
- Existing route aliases remain mounted: `/api/auth/*` and `/api/v1/auth/*`.
- Treat `X-Tenant-ID` and `X-Tenant-Slug` as non-authoritative. Keep the existing explicit `X-Tenant-Override` platform behavior limited to SUPER_ADMIN after target validation.
- `X-Branch-ID` selects a branch only after server-side existence, tenant, role and membership checks.
- On each protected request, confirm the token subject still exists and current role/tenant state still agrees with its claims; revoked or stale scope is denied.
- Denials must not disclose another tenant's or branch's resource existence. Reuse the current API error conventions without creating a second auth contract.

## Existing Auth and Bootstrap Routes

| Existing route(s) | Actor / scope contract |
|---|---|
| `POST /api/auth/register-super-admin`, `POST /api/v1/auth/register-super-admin` | Existing setup operation only. Requires `X-Initial-Admin-Bootstrap-Token`, validated against the owner-provided `INITIAL_ADMIN_BOOTSTRAP_TOKEN` while persistent bootstrap state is unused; exactly one successful operation may create the first tenant/SUPER_ADMIN. Repeated or concurrent losers are denied. No default proof or account credentials. The proof is not logged or persisted. |
| `POST /api/auth/login`, `POST /api/v1/auth/login` | Existing credentials and optional tenant ID contract remains. Auth lookup and login-attempt controls use narrowly scoped system/auth access; errors remain non-enumerating. |
| `POST /api/auth/refresh`, `POST /api/v1/auth/refresh` | Existing refresh-token contract remains scoped to the token's owning user and tenant. |
| `POST /api/auth/logout`, `POST /api/v1/auth/logout`; `GET /api/auth/me`, `GET /api/v1/auth/me` | Existing authenticated actor; revocation/current-user reads must bind to validated token and user, not arbitrary tenant headers. |
| `POST /api/auth/tenant-admins`, `/api/v1/auth/tenant-admins`; `POST /api/auth/users`, `/api/v1/auth/users` | Preserve existing SUPER_ADMIN/TENANT_ADMIN role checks and target-tenant restrictions. No operational user may create privileged users by manipulating tenant fields. |

## Existing Protected Business Routes

| Existing surface | Required server-side behavior |
|---|---|
| `/api/locations` and `/api/users/:user_id/branches` | List only authorized tenant/branch locations. A user with no assignments cannot see all locations by omission. Branch assignment writes validate target user and location in the actor's effective tenant. |
| `/api/v1/menu` and `/api/v1/menu/*` | All category/product/modifier/topping reads and writes use the actor's effective tenant; branch-sensitive catalog and override operations also enforce authorized branch scope. IDs from another tenant disclose nothing and cause no mutation. |
| `/api/v1/orders`, `/api/v1/orders/catalog`, `/api/v1/orders/summary`, `/api/v1/orders/:order_id`, `/api/v1/orders/:order_id/status` | Use the actor's effective tenant and authorized branch. Order/catalog queries cannot rely on an ordinary tenant header. Order item/option reads retain parent order scope. Creation uses server-derived branch price/availability and rejects unavailable products. |
| `/api/platform/tenants` | Remains restricted to SUPER_ADMIN. Cross-tenant operation is allowed only through the existing explicit platform override, validated target and endpoint permission. |

## Branch Role Semantics

- `BRANCH_MANAGER`, `CAJERO`, `MESERO`, and `COCINERO` are branch-scoped by the existing domain method `User::requires_explicit_branch()`.
- No assignment means no branch access for these roles. A caller-supplied UUID cannot create assignment.
- An assigned actor may request only a currently assigned branch.
- Tenant or branch administrators may access multiple branches only to the extent allowed by their existing permissions and explicit effective scope; an empty assignment list is not, by itself, authorization.
- A branch supplied together with a tenant override must belong to the effective override tenant; any user assignment restrictions still apply.

## Failure Conditions

Requests fail closed for missing/invalid authentication, insufficient role/permission, absent or revoked branch assignment, cross-tenant resource identifiers, invalid override/branch combinations, missing required database context, and failed membership lookups. Error responses must not distinguish another tenant's hidden record from a nonexistent record when doing so would disclose cross-scope information.

## Release Contract

The image publication operation is permitted only after all mandatory checks in the feature plan succeed. A failed, cancelled, unavailable or skipped mandatory validation job blocks publication.
