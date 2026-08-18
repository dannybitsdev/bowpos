import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { MenuPage } from './MenuPage';
import { sortMenu } from './menuUtils';

vi.mock('../features/auth/infrastructure/http/apiClient', () => ({
  default: { get: vi.fn().mockResolvedValue({ data: { data: [{ id: 'category', name: 'Platos', display_order: 1, products: [] }] } }) },
}));

describe('MenuPage', () => {
  it('orders products alphabetically within each category', () => {
    const menu = sortMenu([{ id: 'category', name: 'Platos', display_order: 1, products: [
      { id: '2', category_id: 'category', name: 'Bandeja', price: 2 },
      { id: '1', category_id: 'category', name: 'Arepa', price: 1 },
    ] }]);

    expect(menu[0].products.map((product) => product.name)).toEqual(['Arepa', 'Bandeja']);
  });

  it('renders the responsive category navigation', async () => {
    render(<MenuPage />);
    expect(await screen.findByRole('tab', { name: 'Todas' })).toBeInTheDocument();
  });
});