export type Product = {
  id: string;
  category_id: string;
  name: string;
  description?: string | null;
  price: number;
  stock?: number;
  image_url?: string | null;
};

export type MenuCategory = {
  id: string;
  name: string;
  description?: string | null;
  image_url?: string | null;
  display_order: number;
  products: Product[];
};

export type MenuResponse = {
  data: MenuCategory[];
};

export type ProductPayload = Omit<Product, 'id'>;

export type CategoryPayload = {
  name: string;
  description: string | null;
  image_url: string | null;
  display_order: number;
};

export type Modifier = {
  id: string;
  modifier_group_id: string;
  name: string;
  price_delta: number;
  is_active: boolean;
};

export type ModifierGroup = {
  id: string;
  name: string;
  required: boolean;
  min_selections: number;
  max_selections: number;
  is_active: boolean;
  modifiers: Modifier[];
};

export type ModifierGroupPayload = {
  name: string;
  required: boolean;
  min_selections: number;
  max_selections: number;
};

export type ModifierPayload = {
  name: string;
  price_delta: number;
  is_active: boolean;
};
