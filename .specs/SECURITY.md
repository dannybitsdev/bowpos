# Security & Authorization Specs

## Authentication & Session
- JWT-based authentication passed via `Authorization: Bearer <token>` or `HttpOnly` same-site cookies.
- Middleware: Axum `AuthUser` extractor enforces authentication on every route by default.

## RBAC Matrix
- **Roles**: SUPER_ADMIN, ADMIN_TENANT, CAJERO, MESERO, COCINERO.
- **Permissions format**: `module:action` (e.g., `ventas:create`, `inventario:admin`, `config:update`).
- **Endpoint Guarding**: Handlers must explicitly specify required permissions via Axum layers/extractors.

## CORS & Proxies
- Explicit CORS origins only (`https://bowpos.onrender.com` / custom tenant subdomains). `allow_credentials(true)` is mandatory.