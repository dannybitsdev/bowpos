import { describe, expect, it } from 'vitest';

import { calculateItemSubtotal, calculateItemUnitPrice, calculateOrderSubtotal, calculateUnitPrice, getSelectedOptions } from './orderUtils';
import type { CatalogProduct, OrderDraftItem } from './orderTypes';

const burger: CatalogProduct = {
  id: 'burger',
  name: 'Hamburguesa',
  price: 20000,
  modifier_groups: [
    {
      id: 'temp-group',
      name: 'Término de carne',
      required: true,
      min_selections: 1,
      max_selections: 1,
      modifiers: [
        { id: 'medium', name: 'Término medio', price: 0 },
        { id: 'well-done', name: 'Bien cocido', price: 0 },
      ],
    },
    {
      id: 'cheese-group',
      name: 'Adicionales',
      required: false,
      min_selections: 0,
      max_selections: 2,
      modifiers: [
        { id: 'extra-cheese', name: 'Extra queso', price: 3000 },
        { id: 'bacon', name: 'Tocineta', price: 4000 },
      ],
    },
  ],
  toppings: [{ id: 'fries', name: 'Papas', price: 5000 }],
};

describe('getSelectedOptions', () => {
  it('returns only the modifiers/toppings whose ids were selected', () => {
    const selected = getSelectedOptions(burger, ['extra-cheese'], ['fries']);
    expect(selected.map((option) => option.id).sort()).toEqual(['extra-cheese', 'fries']);
  });

  it('returns an empty list when nothing is selected', () => {
    expect(getSelectedOptions(burger, [], [])).toEqual([]);
  });
});

describe('calculateUnitPrice', () => {
  it('equals the base price when no modifiers or toppings are selected', () => {
    expect(calculateUnitPrice(burger, [], [])).toBe(20000);
  });

  it('adds the price delta of a free modifier without changing the total', () => {
    expect(calculateUnitPrice(burger, ['medium'], [])).toBe(20000);
  });

  it('adds the price delta of paid modifiers and toppings', () => {
    expect(calculateUnitPrice(burger, ['medium', 'extra-cheese'], ['fries'])).toBe(20000 + 3000 + 5000);
  });

  it('sums multiple paid modifiers from the same group', () => {
    expect(calculateUnitPrice(burger, ['extra-cheese', 'bacon'], [])).toBe(20000 + 3000 + 4000);
  });
});

describe('calculateItemUnitPrice / calculateItemSubtotal', () => {
  const item: OrderDraftItem = {
    id: 'item-1',
    product: burger,
    quantity: 3,
    modifierIds: ['medium', 'extra-cheese'],
    toppingIds: ['fries'],
    notes: '',
  };

  it('calculates the unit price including customizations', () => {
    expect(calculateItemUnitPrice(item)).toBe(20000 + 3000 + 5000);
  });

  it('multiplies the unit price by the quantity for the subtotal', () => {
    expect(calculateItemSubtotal(item)).toBe((20000 + 3000 + 5000) * 3);
  });
});

describe('calculateOrderSubtotal', () => {
  it('sums the subtotal of every item in the cart', () => {
    const items: OrderDraftItem[] = [
      { id: '1', product: burger, quantity: 1, modifierIds: ['medium'], toppingIds: [], notes: '' },
      { id: '2', product: burger, quantity: 2, modifierIds: ['medium', 'bacon'], toppingIds: ['fries'], notes: '' },
    ];

    expect(calculateOrderSubtotal(items)).toBe(20000 + (20000 + 4000 + 5000) * 2);
  });

  it('returns 0 for an empty cart', () => {
    expect(calculateOrderSubtotal([])).toBe(0);
  });
});
