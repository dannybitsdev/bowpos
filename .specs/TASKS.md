# Development Tasks & Project Roadmap

## Completed Features
- [x] Monorepo structure setup (`/backend`, `/frontend`, `/infra`).
- [x] Infrastructure: Nginx reverse proxy configuration unifying `/api/` and frontend SPA routes.
- [x] Infrastructure: Traefik v3 setup for HTTPS dynamic tenant wildcard certificates (`*.your-domain.com`).
- [x] Frontend UI: "Sabor & Raíz" dark theme palette and responsive component shell.
- [x] System UX: Replacement of native `window.alert()` with custom `ConfirmationModal` pattern.

## Active Sprint / In Progress
- [ ] **Authentication & Session Persistence**:
  - [ ] Resolve CORS and `401 Unauthorized` issue on `/api/locations` under Render production environment.
  - [ ] Enforce explicit `CorsLayer` in Axum with `.allow_credentials(true)` and domain origin matching.
  - [ ] Ensure frontend API client sends `Authorization: Bearer <token>` and `withCredentials: true` consistently.
  - [ ] Prevent race condition in `ProtectedRoute.tsx` on initial page load.

## Backlog / Upcoming Tasks
- [ ] **Product Modifiers CRUD**:
  - [ ] Implement backend tables and SQLx queries for Modifier Groups and items.
  - [ ] Implement frontend Back-Office views for managing product add-ons.
  - [ ] Integrate modifier choice modal into the POS order creation panel.
- [ ] **Tablet Layout Optimization**:
  - [ ] Fix product card stretching on viewports between 768px and 1024px using fixed aspect ratios (`aspect-square` / `aspect-[4/3]`).
- [ ] **Automated Testing & CI/CD**:
  - [ ] Configure GitHub Actions workflow running `cargo test`, `pnpm test`, and `pnpm lint`.