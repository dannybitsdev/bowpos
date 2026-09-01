import { fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { ModifierFormModal } from './ModifierFormModal';

describe('ModifierFormModal', () => {
  it('submits a new modifier payload with its price delta', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn().mockResolvedValue(undefined);
    render(<ModifierFormModal modifier={null} groupName="Adicionales" onClose={vi.fn()} onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText('Nombre'), 'Extra queso');
    await user.clear(screen.getByLabelText('Costo adicional'));
    await user.type(screen.getByLabelText('Costo adicional'), '3000');
    await user.click(screen.getByRole('button', { name: 'Guardar modificador' }));

    expect(onSubmit).toHaveBeenCalledWith({ name: 'Extra queso', price_delta: 3000, is_active: true });
  });

  it('rejects a negative price delta', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<ModifierFormModal modifier={null} groupName="Adicionales" onClose={vi.fn()} onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText('Nombre'), 'Extra queso');
    fireEvent.change(screen.getByLabelText('Costo adicional'), { target: { value: '-100' } });
    await user.click(screen.getByRole('button', { name: 'Guardar modificador' }));

    expect(screen.getByText(/costo adicional válido/i)).toBeInTheDocument();
    expect(onSubmit).not.toHaveBeenCalled();
  });
});
