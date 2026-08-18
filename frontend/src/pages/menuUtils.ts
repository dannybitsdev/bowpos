import type { MenuCategory } from './menuTypes';

export function sortMenu(categories: MenuCategory[]) {
  return [...categories]
    .sort((a, b) => a.display_order - b.display_order || a.name.localeCompare(b.name, 'es'))
    .map((category) => ({
      ...category,
      products: [...category.products].sort((a, b) => a.name.localeCompare(b.name, 'es')),
    }));
}