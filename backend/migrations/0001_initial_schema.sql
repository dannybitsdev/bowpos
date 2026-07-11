CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sedes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    nombre VARCHAR(255) NOT NULL,
    direccion TEXT NOT NULL,
    ciudad VARCHAR(120) NOT NULL,
    configuracion_impresora JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS config_ui (
    tenant_id UUID PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    color_primario VARCHAR(20) NOT NULL,
    color_secundario VARCHAR(20) NOT NULL,
    color_fondo VARCHAR(20) NOT NULL,
    tipografia VARCHAR(100) NOT NULL,
    logo_url TEXT
);

CREATE TABLE IF NOT EXISTS usuarios (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    sede_id UUID NULL REFERENCES sedes(id) ON DELETE SET NULL,
    nombre VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    rol VARCHAR(30) NOT NULL CHECK (rol IN ('SUPER_ADMIN','ADMIN_TENANT','CAJERO','MESERO'))
);

CREATE TABLE IF NOT EXISTS productos (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    nombre VARCHAR(255) NOT NULL,
    precio NUMERIC(12,2) NOT NULL DEFAULT 0,
    stock INTEGER NOT NULL DEFAULT 0,
    imagen_url TEXT
);

CREATE INDEX IF NOT EXISTS idx_sedes_tenant_id ON sedes (tenant_id);
CREATE INDEX IF NOT EXISTS idx_usuarios_tenant_id ON usuarios (tenant_id);
CREATE INDEX IF NOT EXISTS idx_productos_tenant_id ON productos (tenant_id);

-- RLS note:
-- En Postgres, para aplicar RLS a nivel de tenant, crear una política por tabla:
-- ALTER TABLE sedes ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY sedes_tenant_policy ON sedes
-- USING (tenant_id = current_setting('app.tenant_id', true)::uuid);
-- La misma idea aplica a usuarios, productos, etc.
