import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ConfirmationModal } from './ConfirmationModal';

const baseProps = {
  open: true,
  title: 'Eliminar producto',
  description: 'Esta acción no se puede deshacer.',
  confirmLabel: 'Eliminar',
  cancelLabel: 'Cancelar',
  variant: 'destructive' as const,
};

describe('ConfirmationModal', () => {
  it('does not render anything when closed', () => {
    render(<ConfirmationModal {...baseProps} open={false} onConfirm={vi.fn()} onCancel={vi.fn()} />);
    expect(screen.queryByRole('alertdialog')).not.toBeInTheDocument();
  });

  it('renders the title, description and triggers onConfirm when the confirm button is clicked', () => {
    const onConfirm = vi.fn();
    const onCancel = vi.fn();
    render(<ConfirmationModal {...baseProps} onConfirm={onConfirm} onCancel={onCancel} />);

    expect(screen.getByText('Eliminar producto')).toBeInTheDocument();
    expect(screen.getByText('Esta acción no se puede deshacer.')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Eliminar' }));

    expect(onConfirm).toHaveBeenCalledTimes(1);
    expect(onCancel).not.toHaveBeenCalled();
  });

  it('triggers onCancel when the cancel button is clicked', () => {
    const onConfirm = vi.fn();
    const onCancel = vi.fn();
    render(<ConfirmationModal {...baseProps} onConfirm={onConfirm} onCancel={onCancel} />);

    fireEvent.click(screen.getByRole('button', { name: 'Cancelar' }));

    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onConfirm).not.toHaveBeenCalled();
  });

  it('triggers onCancel when clicking the backdrop, but not when clicking inside the dialog', () => {
    const onConfirm = vi.fn();
    const onCancel = vi.fn();
    render(<ConfirmationModal {...baseProps} onConfirm={onConfirm} onCancel={onCancel} />);

    fireEvent.click(screen.getByText('Eliminar producto'));
    expect(onCancel).not.toHaveBeenCalled();

    fireEvent.click(screen.getByTestId('confirmation-modal-backdrop'));
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it('closes on Escape and confirms on Enter', () => {
    const onConfirm = vi.fn();
    const onCancel = vi.fn();
    render(<ConfirmationModal {...baseProps} onConfirm={onConfirm} onCancel={onCancel} />);

    fireEvent.keyDown(document, { key: 'Escape' });
    expect(onCancel).toHaveBeenCalledTimes(1);

    fireEvent.keyDown(document, { key: 'Enter' });
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it('moves focus to the confirm button on open (state cleanup / focus management)', () => {
    const onConfirm = vi.fn();
    const onCancel = vi.fn();
    render(<ConfirmationModal {...baseProps} onConfirm={onConfirm} onCancel={onCancel} />);

    expect(screen.getByRole('button', { name: 'Eliminar' })).toHaveFocus();
  });
});
