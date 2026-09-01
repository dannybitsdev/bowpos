import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { ModifierGroupFormModal } from './ModifierGroupFormModal';

describe('ModifierGroupFormModal', () => {
  it('submits a new modifier group payload', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn().mockResolvedValue(undefined);
    render(<ModifierGroupFormModal group={null} onClose={vi.fn()} onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText('Nombre del grupo'), 'Adicionales');
    await user.click(screen.getByLabelText('Selección obligatoria'));
    await user.click(screen.getByRole('button', { name: 'Guardar grupo' }));

    expect(onSubmit).toHaveBeenCalledWith({ name: 'Adicionales', required: true, min_selections: 0, max_selections: 1 });
  });

  it('rejects a max selection lower than the minimum', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<ModifierGroupFormModal group={null} onClose={vi.fn()} onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText('Nombre del grupo'), 'Bebidas');
    await user.clear(screen.getByLabelText('Mínimo de selecciones'));
    await user.type(screen.getByLabelText('Mínimo de selecciones'), '3');
    await user.clear(screen.getByLabelText('Máximo de selecciones'));
    await user.type(screen.getByLabelText('Máximo de selecciones'), '1');
    await user.click(screen.getByRole('button', { name: 'Guardar grupo' }));

    expect(onSubmit).not.toHaveBeenCalled();
    expect(screen.getByText(/mínimo ≤ máximo/i)).toBeInTheDocument();
  });
});
