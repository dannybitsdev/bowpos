ALTER TABLE categories ADD COLUMN IF NOT EXISTS image_url VARCHAR(2048);

CREATE INDEX IF NOT EXISTS idx_categories_tenant ON categories (tenant_id);
CREATE INDEX IF NOT EXISTS idx_products_tenant_category ON products (tenant_id, category_id);