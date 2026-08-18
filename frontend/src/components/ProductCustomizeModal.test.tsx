import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ProductCustomizeModal } from './ProductCustomizeModal';
import type { CatalogProduct } from '../pages/orderTypes';

const product: CatalogProduct = {
  id: 'burger', name: 'Hamburguesa', price: 20000, image_url: null,
  modifier_groups: [{ id: 'term', name: 'Término', required: true, min_selections: 1, max_selections: 1, modifiers: [{ id: 'medium', name: 'Término medio', price: 0 }] }],
  toppings: [{ id: 'cheese', name: 'Queso extra', price: 2500 }],
};

describe('ProductCustomizeModal', () => {
  it('requires mandatory modifiers and calculates selected extras', async () => {
    const user = userEvent.setup();
    const onSave = vi.fn();
    render(<ProductCustomizeModal product={product} onClose={vi.fn()} onSave={onSave} />);

    await user.click(screen.getByRole('button', { name: 'Agregar al pedido' }));
    expect(screen.getByText('Selecciona una opción en Término.')).toBeInTheDocument();
    await user.click(screen.getByLabelText('Término medio'));
    await user.click(screen.getByRole('checkbox'));
    expect(screen.getByText(/\$.*22\.500/)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Agregar al pedido' }));

    expect(onSave).toHaveBeenCalledWith(expect.objectContaining({ modifierIds: ['medium'], toppingIds: ['cheese'] }));
  });
});
