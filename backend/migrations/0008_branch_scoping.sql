-- Fase 1: rol BRANCH_MANAGER
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_check;
ALTER TABLE users ADD CONSTRAINT users_role_check
    CHECK (role IN ('SUPER_ADMIN','ADMIN_TENANT','BRANCH_MANAGER','CAJERO','MESERO'));

ALTER TABLE users DROP CONSTRAINT IF EXISTS users_tenant_id_unique;
ALTER TABLE users ADD CONSTRAINT users_tenant_id_unique UNIQUE (id, tenant_id);

-- Fase 1: acceso de usuario a una o varias sedes (reemplaza el uso exclusivo de users.location_id)
CREATE TABLE IF NOT EXISTS user_branch_access (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    location_id UUID NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, user_id, location_id),
    FOREIGN KEY (user_id, tenant_id) REFERENCES users(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (location_id, tenant_id) REFERENCES locations(id, tenant_id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_user_branch_primary
    ON user_branch_access (tenant_id, user_id) WHERE is_primary;
CREATE INDEX IF NOT EXISTS idx_user_branch_location
    ON user_branch_access (tenant_id, location_id);

-- Backfill: usuarios que ya tenían una sede asignada vía users.location_id
INSERT INTO user_branch_access (tenant_id, user_id, location_id, is_primary)
SELECT tenant_id, id, location_id, TRUE
FROM users
WHERE location_id IS NOT NULL
ON CONFLICT DO NOTHING;

-- Fase 3: catálogo con overrides de precio/stock/disponibilidad por sede
CREATE TABLE IF NOT EXISTS branch_product_overrides (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL,
    product_id UUID NOT NULL,
    price NUMERIC(10,2) CHECK (price IS NULL OR price >= 0),
    stock INTEGER CHECK (stock IS NULL OR stock >= 0),
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, location_id, product_id),
    FOREIGN KEY (location_id, tenant_id) REFERENCES locations(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (product_id, tenant_id) REFERENCES products(id, tenant_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_branch_overrides_tenant_location
    ON branch_product_overrides(tenant_id, location_id);

-- Fase 1/3: orders.location_id pasa a ser obligatorio (ya viene poblado desde 0007)
ALTER TABLE orders ALTER COLUMN location_id SET NOT NULL;

CREATE INDEX IF NOT EXISTS idx_orders_tenant_branch_status
    ON orders(tenant_id, location_id, status);
CREATE INDEX IF NOT EXISTS idx_orders_tenant_branch_created
    ON orders(tenant_id, location_id, created_at DESC);

-- Fase 4: Row Level Security como defensa en profundidad sobre orders.
-- El filtrado real ocurre en Rust (fuente de verdad); RLS es un backstop.
-- Cuando no se fija app.tenant_id (migraciones/seeder/pool sin scope) se permite todo (fail-open
-- para no romper procesos internos), pero cuando el backend fija la sesión por request, se aplica.
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS orders_tenant_branch_policy ON orders;
CREATE POLICY orders_tenant_branch_policy ON orders
    USING (
        current_setting('app.tenant_id', true) IS NULL
        OR (
            tenant_id = current_setting('app.tenant_id', true)::uuid
            AND (
                current_setting('app.branch_scope', true) IS NULL
                OR current_setting('app.branch_scope', true) = 'ALL'
                OR location_id = current_setting('app.branch_scope', true)::uuid
            )
        )
    );
