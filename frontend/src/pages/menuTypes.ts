export type Product = {
  id: string;
  category_id: string;
  name: string;
  description?: string | null;
  price: number;
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