ALTER TABLE orders ADD COLUMN IF NOT EXISTS order_number BIGINT;

WITH numbered_orders AS (
    SELECT id, ROW_NUMBER() OVER (PARTITION BY tenant_id ORDER BY created_at ASC, id ASC) AS number
    FROM orders
)
UPDATE orders
SET order_number = numbered_orders.number
FROM numbered_orders
WHERE orders.id = numbered_orders.id
  AND orders.order_number IS NULL;

ALTER TABLE orders ALTER COLUMN order_number SET NOT NULL;
ALTER TABLE orders ALTER COLUMN order_number SET DEFAULT 0;

CREATE UNIQUE INDEX IF NOT EXISTS idx_orders_tenant_order_number
    ON orders(tenant_id, order_number);
