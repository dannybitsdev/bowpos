import { create } from 'zustand';

import type { ConfirmationOptions, ResolvedConfirmationOptions } from '../domain/modalTypes';

const DEFAULT_LABELS = {
  confirmLabel: 'Confirmar',
  cancelLabel: 'Cancelar',
  variant: 'info' as const,
};

type ModalState = {
  isOpen: boolean;
  options: ResolvedConfirmationOptions | null;
  resolver: ((confirmed: boolean) => void) | null;
  /** Opens the modal and resolves once the user confirms or cancels/dismisses it. */
  request: (options: ConfirmationOptions) => Promise<boolean>;
  confirm: () => void;
  cancel: () => void;
};

export const useModalStore = create<ModalState>((set, get) => ({
  isOpen: false,
  options: null,
  resolver: null,
  request: (options) =>
    new Promise<boolean>((resolve) => {
      // Any modal already pending is dismissed (as "cancelled") in favor of the new request.
      get().resolver?.(false);
      set({
        isOpen: true,
        options: { ...DEFAULT_LABELS, ...options },
        resolver: resolve,
      });
    }),
  confirm: () => {
    get().resolver?.(true);
    set({ isOpen: false, options: null, resolver: null });
  },
  cancel: () => {
    get().resolver?.(false);
    set({ isOpen: false, options: null, resolver: null });
  },
}));
