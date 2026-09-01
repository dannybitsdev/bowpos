import { useState, type FormEvent } from 'react';

import type { ModifierGroup, ModifierGroupPayload } from '../pages/menuTypes';

type ModifierGroupFormModalProps = {
  group: ModifierGroup | null;
  onClose: () => void;
  onSubmit: (payload: ModifierGroupPayload) => Promise<void>;
};

export function ModifierGroupFormModal({ group, onClose, onSubmit }: ModifierGroupFormModalProps) {
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const name = String(form.get('name') ?? '').trim();
    const required = form.get('required') === 'on';
    const minSelections = Number(form.get('min_selections'));
    const maxSelections = Number(form.get('max_selections'));

    if (!name || Number.isNaN(minSelections) || Number.isNaN(maxSelections) || minSelections < 0 || maxSelections < minSelections) {
      setError('Completa los campos con valores válidos (mínimo ≤ máximo).');
      return;
    }

    setSaving(true);
    setError(null);
    try {
      await onSubmit({ name, required, min_selections: minSelections, max_selections: maxSelections });
      onClose();
    } catch {
      setError('No fue posible guardar el grupo de modificadores.');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex h-[100dvh] items-end justify-center overflow-y-auto bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="modifier-group-form-title">
      <form onSubmit={(event) => void handleSubmit(event)} className="my-auto max-h-[calc(100dvh-1rem)] w-full max-w-lg space-y-4 overflow-y-auto rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl sm:max-h-[calc(100dvh-2rem)] sm:rounded-3xl sm:p-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">Modificadores</p>
            <h2 id="modifier-group-form-title" className="mt-1 text-2xl font-semibold text-white">{group ? 'Editar grupo' : 'Nuevo grupo'}</h2>
          </div>
          <button type="button" onClick={onClose} className="text-2xl text-[var(--color-muted)]" aria-label="Cerrar formulario">×</button>
        </div>
        <label className="block text-sm text-[var(--color-muted)]">
          Nombre del grupo
          <input name="name" defaultValue={group?.name ?? ''} required maxLength={120} placeholder="Adicionales, Término de carne..." className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" />
        </label>
        <label className="flex items-center gap-3 text-sm text-[var(--color-muted)]">
          <input type="checkbox" name="required" defaultChecked={group?.required ?? false} className="h-4 w-4" />
          Selección obligatoria
        </label>
        <div className="grid gap-4 sm:grid-cols-2">
          <label className="block text-sm text-[var(--color-muted)]">
            Mínimo de selecciones
            <input name="min_selections" type="number" min="0" step="1" defaultValue={group?.min_selections ?? 0} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" />
          </label>
          <label className="block text-sm text-[var(--color-muted)]">
            Máximo de selecciones
            <input name="max_selections" type="number" min="1" step="1" defaultValue={group?.max_selections ?? 1} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" />
          </label>
        </div>
        {error ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</p> : null}
        <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end">
          <button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)] hover:text-white">Cancelar</button>
          <button type="submit" disabled={saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Guardando...' : 'Guardar grupo'}</button>
        </div>
      </form>
    </div>
  );
}
