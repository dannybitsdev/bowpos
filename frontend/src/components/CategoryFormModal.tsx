import { useEffect, useState, type FormEvent } from 'react';

import type { CategoryPayload, MenuCategory } from '../pages/menuTypes';

type CategoryFormModalProps = {
  category: MenuCategory | null;
  onClose: () => void;
  onSubmit: (payload: CategoryPayload) => Promise<void>;
};

const emptyPayload: CategoryPayload = { name: '', description: null, image_url: null, display_order: 0 };

export function CategoryFormModal({ category, onClose, onSubmit }: CategoryFormModalProps) {
  const [payload, setPayload] = useState<CategoryPayload>(emptyPayload);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setPayload(category ? { name: category.name, description: category.description ?? null, image_url: category.image_url ?? null, display_order: category.display_order } : emptyPayload);
  }, [category]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!payload.name.trim()) { setError('El nombre es obligatorio.'); return; }
    setSaving(true);
    setError(null);
    try { await onSubmit({ ...payload, name: payload.name.trim(), description: payload.description?.trim() || null, image_url: payload.image_url?.trim() || null }); onClose(); }
    catch { setError('No fue posible guardar la categoría.'); }
    finally { setSaving(false); }
  }

  return <div className="fixed inset-0 z-50 flex items-end justify-center overflow-y-auto bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="category-form-title">
    <form onSubmit={handleSubmit} className="max-h-[100dvh] w-full max-w-xl space-y-4 overflow-y-auto rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-2xl sm:max-h-[calc(100dvh-2rem)] sm:rounded-3xl sm:p-6">
      <div className="flex items-start justify-between gap-4"><div><p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">Catálogo</p><h2 id="category-form-title" className="mt-1 text-2xl font-semibold text-white">{category ? 'Editar categoría' : 'Nueva categoría'}</h2></div><button type="button" onClick={onClose} className="text-2xl text-[var(--color-muted)]" aria-label="Cerrar formulario">×</button></div>
      <label className="block text-sm text-[var(--color-muted)]">Nombre<input value={payload.name} onChange={(event) => setPayload({ ...payload, name: event.target.value })} required maxLength={255} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label>
      <label className="block text-sm text-[var(--color-muted)]">Descripción<textarea value={payload.description ?? ''} onChange={(event) => setPayload({ ...payload, description: event.target.value })} rows={3} className="mt-2 w-full resize-y rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label>
      <div className="grid gap-4 sm:grid-cols-2"><label className="block text-sm text-[var(--color-muted)]">Orden<input type="number" min="0" value={payload.display_order} onChange={(event) => setPayload({ ...payload, display_order: Number(event.target.value) })} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label><label className="block text-sm text-[var(--color-muted)]">URL de imagen<input type="url" value={payload.image_url ?? ''} onChange={(event) => setPayload({ ...payload, image_url: event.target.value })} placeholder="https://ejemplo.com/categoria.jpg" className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label></div>
      {error ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</p> : null}
      <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end"><button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)]">Cancelar</button><button type="submit" disabled={saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Guardando...' : 'Guardar categoría'}</button></div>
    </form>
  </div>;
}
