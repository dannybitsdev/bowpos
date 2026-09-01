import { act, fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { useModalStore } from '../application/modalStore';
import { ConfirmationModalHost } from './ConfirmationModalHost';

describe('ConfirmationModalHost + modalStore', () => {
  it('resolves true and resets state when confirmed', async () => {
    render(<ConfirmationModalHost />);
    expect(screen.queryByRole('alertdialog')).not.toBeInTheDocument();

    let result: Promise<boolean> | undefined;
    act(() => {
      result = useModalStore.getState().request({ title: 'Cerrar sesión', variant: 'warning' });
    });

    expect(screen.getByRole('alertdialog')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Confirmar' }));

    await expect(result).resolves.toBe(true);
    expect(useModalStore.getState().isOpen).toBe(false);
    expect(useModalStore.getState().options).toBeNull();
    expect(screen.queryByRole('alertdialog')).not.toBeInTheDocument();
  });

  it('resolves false and resets state when cancelled', async () => {
    render(<ConfirmationModalHost />);

    let result: Promise<boolean> | undefined;
    act(() => {
      result = useModalStore.getState().request({ title: 'Eliminar producto', variant: 'destructive' });
    });

    fireEvent.click(screen.getByRole('button', { name: 'Cancelar' }));

    await expect(result).resolves.toBe(false);
    expect(useModalStore.getState().isOpen).toBe(false);
  });
});
