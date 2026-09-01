import type { CatalogOption, CatalogProduct, OrderDraftItem } from './orderTypes';

/** All selectable options (modifiers + toppings) a product exposes, flattened. */
function allSelectableOptions(product: CatalogProduct): CatalogOption[] {
  return [...product.modifier_groups.flatMap((group) => group.modifiers), ...product.toppings];
}

export function getSelectedOptions(product: CatalogProduct, modifierIds: string[], toppingIds: string[]): CatalogOption[] {
  return allSelectableOptions(product).filter((option) => modifierIds.includes(option.id) || toppingIds.includes(option.id));
}

/** Base product price plus the price delta of every selected modifier/topping. */
export function calculateUnitPrice(product: CatalogProduct, modifierIds: string[], toppingIds: string[]): number {
  return product.price + getSelectedOptions(product, modifierIds, toppingIds).reduce((sum, option) => sum + option.price, 0);
}

export function calculateItemUnitPrice(item: OrderDraftItem): number {
  return calculateUnitPrice(item.product, item.modifierIds, item.toppingIds);
}

export function calculateItemSubtotal(item: OrderDraftItem): number {
  return calculateItemUnitPrice(item) * item.quantity;
}

export function calculateOrderSubtotal(items: OrderDraftItem[]): number {
  return items.reduce((sum, item) => sum + calculateItemSubtotal(item), 0);
}
