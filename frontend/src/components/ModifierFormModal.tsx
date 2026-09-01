import { useState, type FormEvent } from 'react';

import type { Modifier, ModifierPayload } from '../pages/menuTypes';

type ModifierFormModalProps = {
  modifier: Modifier | null;
  groupName: string;
  onClose: () => void;
  onSubmit: (payload: ModifierPayload) => Promise<void>;
};

export function ModifierFormModal({ modifier, groupName, onClose, onSubmit }: ModifierFormModalProps) {
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const name = String(form.get('name') ?? '').trim();
    const priceDelta = Number(form.get('price_delta'));
    const isActive = form.get('is_active') === 'on';

    if (!name || Number.isNaN(priceDelta) || priceDelta < 0) {
      setError('Completa un nombre y un costo adicional válido (mayor o igual a cero).');
      return;
    }

    setSaving(true);
    setError(null);
    try {
      await onSubmit({ name, price_delta: priceDelta, is_active: isActive });
      onClose();
    } catch {
      setError('No fue posible guardar el modificador.');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex h-[100dvh] items-end justify-center overflow-y-auto bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="modifier-form-title">
      <form onSubmit={(event) => void handleSubmit(event)} className="my-auto max-h-[calc(100dvh-1rem)] w-full max-w-md space-y-4 overflow-y-auto rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl sm:max-h-[calc(100dvh-2rem)] sm:rounded-3xl sm:p-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">{groupName}</p>
            <h2 id="modifier-form-title" className="mt-1 text-2xl font-semibold text-white">{modifier ? 'Editar modificador' : 'Nuevo modificador'}</h2>
          </div>
          <button type="button" onClick={onClose} className="text-2xl text-[var(--color-muted)]" aria-label="Cerrar formulario">×</button>
        </div>
        <label className="block text-sm text-[var(--color-muted)]">
          Nombre
          <input name="name" defaultValue={modifier?.name ?? ''} required maxLength={120} placeholder="Extra queso, Sin cebolla..." className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" />
        </label>
        <label className="block text-sm text-[var(--color-muted)]">
          Costo adicional
          <input name="price_delta" type="number" step="0.01" defaultValue={modifier?.price_delta ?? 0} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" />
        </label>
        <label className="flex items-center gap-3 text-sm text-[var(--color-muted)]">
          <input type="checkbox" name="is_active" defaultChecked={modifier?.is_active ?? true} className="h-4 w-4" />
          Disponible
        </label>
        {error ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</p> : null}
        <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end">
          <button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)] hover:text-white">Cancelar</button>
          <button type="submit" disabled={saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Guardando...' : 'Guardar modificador'}</button>
        </div>
      </form>
    </div>
  );
}
