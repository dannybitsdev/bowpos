import { render, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { ModifierGroupsPage } from './ModifierGroupsPage';
import { ConfirmationModalHost } from '../features/modal/presentation/ConfirmationModalHost';

const modifierGroup = {
  id: 'group-1',
  name: 'Adicionales',
  required: false,
  min_selections: 0,
  max_selections: 2,
  is_active: true,
  modifiers: [
    { id: 'modifier-1', modifier_group_id: 'group-1', name: 'Extra queso', price_delta: 3000, is_active: true },
  ],
};

vi.mock('../features/auth/infrastructure/http/apiClient', () => ({
  default: {
    get: vi.fn((url: string) => {
      if (url === '/v1/menu/modifier-groups') return Promise.resolve({ data: { data: [modifierGroup] } });
      if (url === '/v1/menu/products') return Promise.resolve({ data: [] });
      return Promise.resolve({ data: { data: [] } });
    }),
    delete: vi.fn().mockResolvedValue({ data: {} }),
    post: vi.fn().mockResolvedValue({ data: {} }),
    put: vi.fn().mockResolvedValue({ data: {} }),
  },
}));

describe('ModifierGroupsPage', () => {
  it('renders modifier groups and deletes a modifier only after confirming the destructive dialog', async () => {
    const user = userEvent.setup();
    const apiClient = (await import('../features/auth/infrastructure/http/apiClient')).default;

    render(
      <>
        <ModifierGroupsPage />
        <ConfirmationModalHost />
      </>
    );

    expect(await screen.findByText('Adicionales')).toBeInTheDocument();
    expect(screen.getByText('Extra queso')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'Eliminar' }));

    const dialog = await screen.findByRole('alertdialog');
    expect(dialog).toBeInTheDocument();
    expect(apiClient.delete).not.toHaveBeenCalled();

    await user.click(within(dialog).getByRole('button', { name: 'Eliminar' }));

    await waitFor(() => expect(apiClient.delete).toHaveBeenCalledWith('/v1/menu/modifiers/modifier-1'));
  });
});
