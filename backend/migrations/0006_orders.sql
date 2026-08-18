CREATE TABLE IF NOT EXISTS modifier_groups (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(120) NOT NULL,
    required BOOLEAN NOT NULL DEFAULT FALSE,
    min_selections INTEGER NOT NULL DEFAULT 0 CHECK (min_selections >= 0),
    max_selections INTEGER NOT NULL DEFAULT 1 CHECK (max_selections >= min_selections),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (tenant_id, id),
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS modifiers (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    modifier_group_id UUID NOT NULL,
    name VARCHAR(120) NOT NULL,
    price NUMERIC(10,2) NOT NULL DEFAULT 0 CHECK (price >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (tenant_id, id),
    FOREIGN KEY (modifier_group_id, tenant_id) REFERENCES modifier_groups(id, tenant_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS toppings (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(120) NOT NULL,
    price NUMERIC(10,2) NOT NULL DEFAULT 0 CHECK (price >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (tenant_id, id),
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS product_modifier_groups (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id UUID NOT NULL,
    modifier_group_id UUID NOT NULL,
    PRIMARY KEY (tenant_id, product_id, modifier_group_id),
    FOREIGN KEY (product_id, tenant_id) REFERENCES products(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_group_id, tenant_id) REFERENCES modifier_groups(id, tenant_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS product_toppings (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id UUID NOT NULL,
    topping_id UUID NOT NULL,
    PRIMARY KEY (tenant_id, product_id, topping_id),
    FOREIGN KEY (product_id, tenant_id) REFERENCES products(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (topping_id, tenant_id) REFERENCES toppings(id, tenant_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID NULL,
    user_id UUID NULL,
    service_type VARCHAR(20) NOT NULL CHECK (service_type IN ('DINE_IN', 'TAKEAWAY', 'DELIVERY')),
    table_name VARCHAR(80),
    customer_name VARCHAR(160),
    notes TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED' CHECK (status IN ('CREATED', 'IN_PREPARATION', 'READY', 'DELIVERED', 'CANCELLED')),
    payment_method VARCHAR(20) CHECK (payment_method IS NULL OR payment_method IN ('CASH', 'CARD', 'TRANSFER')),
    subtotal NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (subtotal >= 0),
    tax NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (tax >= 0),
    tip NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (tip >= 0),
    discount NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (discount >= 0),
    total NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (total >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, id),
    FOREIGN KEY (location_id, tenant_id) REFERENCES locations(id, tenant_id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(12,2) NOT NULL CHECK (unit_price >= 0),
    notes TEXT,
    subtotal NUMERIC(12,2) NOT NULL CHECK (subtotal >= 0),
    UNIQUE (tenant_id, id),
    FOREIGN KEY (order_id, tenant_id) REFERENCES orders(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (product_id, tenant_id) REFERENCES products(id, tenant_id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS order_item_modifiers (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_item_id UUID NOT NULL,
    modifier_id UUID NOT NULL,
    name VARCHAR(120) NOT NULL,
    price NUMERIC(12,2) NOT NULL CHECK (price >= 0),
    UNIQUE (tenant_id, id),
    FOREIGN KEY (order_item_id, tenant_id) REFERENCES order_items(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_id, tenant_id) REFERENCES modifiers(id, tenant_id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS order_item_toppings (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_item_id UUID NOT NULL,
    topping_id UUID NOT NULL,
    name VARCHAR(120) NOT NULL,
    price NUMERIC(12,2) NOT NULL CHECK (price >= 0),
    UNIQUE (tenant_id, id),
    FOREIGN KEY (order_item_id, tenant_id) REFERENCES order_items(id, tenant_id) ON DELETE CASCADE,
    FOREIGN KEY (topping_id, tenant_id) REFERENCES toppings(id, tenant_id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_orders_tenant_status ON orders(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_orders_tenant_created ON orders(tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_order_items_tenant_order ON order_items(tenant_id, order_id);
CREATE INDEX IF NOT EXISTS idx_modifiers_tenant_group ON modifiers(tenant_id, modifier_group_id);
CREATE INDEX IF NOT EXISTS idx_product_modifier_groups_tenant_product ON product_modifier_groups(tenant_id, product_id);
CREATE INDEX IF NOT EXISTS idx_product_toppings_tenant_product ON product_toppings(tenant_id, product_id);
