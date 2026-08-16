CREATE UNIQUE INDEX IF NOT EXISTS locations_tenant_id_id_idx ON locations (tenant_id, id);

ALTER TABLE users DROP CONSTRAINT IF EXISTS users_location_id_fkey;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_location_tenant_fkey;
ALTER TABLE users ADD CONSTRAINT users_location_tenant_fkey
    FOREIGN KEY (location_id, tenant_id) REFERENCES locations (id, tenant_id) ON DELETE SET NULL;

ALTER TABLE auth_refresh_tokens DROP CONSTRAINT IF EXISTS auth_refresh_tokens_user_id_fkey;
ALTER TABLE auth_refresh_tokens ADD CONSTRAINT auth_refresh_tokens_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
