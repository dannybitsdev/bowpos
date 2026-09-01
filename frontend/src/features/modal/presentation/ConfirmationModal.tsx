import { useEffect, useRef } from 'react';

import type { ConfirmationVariant } from '../domain/modalTypes';

type ConfirmationModalProps = {
  open: boolean;
  title: string;
  description?: string;
  confirmLabel: string;
  cancelLabel: string;
  variant: ConfirmationVariant;
  onConfirm: () => void;
  onCancel: () => void;
};

const variantStyles: Record<ConfirmationVariant, { badge: string; badgeLabel: string; confirmButton: string }> = {
  destructive: {
    badge: 'border-rose-400/40 bg-rose-400/10 text-rose-200',
    badgeLabel: 'Acción destructiva',
    confirmButton: 'bg-rose-500 text-white hover:bg-rose-400',
  },
  warning: {
    badge: 'border-amber-400/40 bg-amber-400/10 text-amber-200',
    badgeLabel: 'Advertencia',
    confirmButton: 'bg-amber-400 text-black hover:bg-amber-300',
  },
  info: {
    badge: 'border-[var(--color-primary)]/40 bg-[var(--color-primary)]/10 text-[var(--color-primary)]',
    badgeLabel: 'Confirmación',
    confirmButton: 'bg-[var(--color-primary)] text-black hover:brightness-95',
  },
};

/** Reusable, accessible confirmation dialog. Purely presentational: state lives in `useModalStore`. */
export function ConfirmationModal({ open, title, description, confirmLabel, cancelLabel, variant, onConfirm, onCancel }: ConfirmationModalProps) {
  const dialogRef = useRef<HTMLDivElement>(null);
  const confirmButtonRef = useRef<HTMLButtonElement>(null);
  const previouslyFocused = useRef<HTMLElement | null>(null);

  useEffect(() => {
    if (!open) return undefined;

    previouslyFocused.current = document.activeElement as HTMLElement | null;
    confirmButtonRef.current?.focus();

    function trapFocus(event: KeyboardEvent) {
      const focusable = dialogRef.current?.querySelectorAll<HTMLElement>('button');
      if (!focusable || focusable.length === 0) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];

      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        event.preventDefault();
        onCancel();
      } else if (event.key === 'Enter') {
        event.preventDefault();
        onConfirm();
      } else if (event.key === 'Tab') {
        trapFocus(event);
      }
    }

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      previouslyFocused.current?.focus();
    };
  }, [open, onConfirm, onCancel]);

  if (!open) return null;

  const styles = variantStyles[variant];

  return (
    <div
      className="fixed inset-0 z-[60] flex items-center justify-center bg-black/70 p-4"
      onClick={onCancel}
      data-testid="confirmation-modal-backdrop"
    >
      <div
        ref={dialogRef}
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="confirmation-modal-title"
        aria-describedby={description ? 'confirmation-modal-description' : undefined}
        onClick={(event) => event.stopPropagation()}
        className="w-full max-w-md rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-6 shadow-2xl"
      >
        <span className={`inline-flex rounded-full border px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.2em] ${styles.badge}`}>
          {styles.badgeLabel}
        </span>
        <h2 id="confirmation-modal-title" className="mt-4 text-xl font-semibold text-white">
          {title}
        </h2>
        {description ? (
          <p id="confirmation-modal-description" className="mt-2 text-sm text-[var(--color-muted)]">
            {description}
          </p>
        ) : null}
        <div className="mt-6 flex flex-col-reverse gap-3 sm:flex-row sm:justify-end">
          <button
            type="button"
            onClick={onCancel}
            className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)] transition hover:text-white"
          >
            {cancelLabel}
          </button>
          <button
            type="button"
            ref={confirmButtonRef}
            onClick={onConfirm}
            className={`rounded-xl px-4 py-2.5 text-sm font-semibold transition ${styles.confirmButton}`}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
