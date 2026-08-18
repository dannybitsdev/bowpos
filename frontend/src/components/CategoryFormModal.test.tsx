import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { CategoryFormModal } from './CategoryFormModal';

describe('CategoryFormModal', () => {
  it('submits a new category payload', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn().mockResolvedValue(undefined);
    render(<CategoryFormModal category={null} onClose={vi.fn()} onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText('Nombre'), 'Postres');
    await user.click(screen.getByRole('button', { name: 'Guardar categoría' }));

    expect(onSubmit).toHaveBeenCalledWith({ name: 'Postres', description: null, image_url: null, display_order: 0 });
  });
});