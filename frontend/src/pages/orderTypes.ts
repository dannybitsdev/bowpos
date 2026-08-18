export type OrderStatus = 'CREATED' | 'IN_PREPARATION' | 'READY' | 'DELIVERED' | 'CANCELLED';
export type ServiceType = 'DINE_IN' | 'TAKEAWAY' | 'DELIVERY';
export type PaymentMethod = 'CASH' | 'CARD' | 'TRANSFER';

export type CatalogOption = { id: string; name: string; price: number };
export type ModifierGroup = { id: string; name: string; required: boolean; min_selections: number; max_selections: number; modifiers: CatalogOption[] };
export type CatalogProduct = { id: string; name: string; price: number; image_url?: string | null; modifier_groups: ModifierGroup[]; toppings: CatalogOption[] };
export type OrderDraftItem = { id: string; product: CatalogProduct; quantity: number; modifierIds: string[]; toppingIds: string[]; notes: string };
export type CreateOrderPayload = { service_type: ServiceType; table_name?: string; customer_name?: string; notes?: string; payment_method?: PaymentMethod; tax_rate: number; tip: number; discount: number; items: Array<{ product_id: string; quantity: number; notes?: string; modifier_ids: string[]; topping_ids: string[] }> };
export type Order = { id: string; order_number: number; tenant_id: string; tenant_name: string; location_name?: string | null; service_type: ServiceType; table_name?: string | null; customer_name?: string | null; notes?: string | null; status: OrderStatus; payment_method?: PaymentMethod | null; subtotal: number; tax: number; tip: number; discount: number; total: number; items: Array<{ id: string; product_id: string; product_name: string; quantity: number; unit_price: number; subtotal: number; notes?: string | null; modifiers: CatalogOption[]; toppings: CatalogOption[] }>; created_at: string };
