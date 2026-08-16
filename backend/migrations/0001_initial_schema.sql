CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    city VARCHAR(120) NOT NULL,
    printer_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS ui_config (
    tenant_id UUID PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    primary_color VARCHAR(20) NOT NULL,
    secondary_color VARCHAR(20) NOT NULL,
    background_color VARCHAR(20) NOT NULL,
    font_family VARCHAR(100) NOT NULL,
    logo_url TEXT
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(30) NOT NULL CHECK (role IN ('SUPER_ADMIN','ADMIN_TENANT','CAJERO','MESERO')),
    CONSTRAINT users_location_tenant_fkey
        FOREIGN KEY (location_id, tenant_id) REFERENCES locations(id, tenant_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_locations_tenant_id ON locations (tenant_id);
CREATE INDEX IF NOT EXISTS idx_users_tenant_id ON users (tenant_id);

-- RLS note:
-- To apply tenant-level RLS in PostgreSQL, create a policy for each table:
-- ALTER TABLE locations ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY locations_tenant_policy ON locations
-- USING (tenant_id = current_setting('app.tenant_id', true)::uuid);
-- The same approach applies to users, categories, products, and other tenant data.
