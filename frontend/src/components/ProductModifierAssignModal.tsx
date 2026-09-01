import { useEffect, useState } from 'react';

import type { ModifierGroup, Product } from '../pages/menuTypes';

type ProductModifierAssignModalProps = {
  products: Product[];
  modifierGroups: ModifierGroup[];
  onLoadAssignedGroupIds: (productId: string) => Promise<string[]>;
  onSave: (productId: string, modifierGroupIds: string[]) => Promise<void>;
  onClose: () => void;
};

export function ProductModifierAssignModal({ products, modifierGroups, onLoadAssignedGroupIds, onSave, onClose }: ProductModifierAssignModalProps) {
  const [selectedProductId, setSelectedProductId] = useState(products[0]?.id ?? '');
  const [selectedGroupIds, setSelectedGroupIds] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!selectedProductId) return;
    setLoading(true);
    onLoadAssignedGroupIds(selectedProductId)
      .then(setSelectedGroupIds)
      .catch(() => setError('No fue posible cargar los grupos asignados.'))
      .finally(() => setLoading(false));
  }, [selectedProductId, onLoadAssignedGroupIds]);

  function toggleGroup(groupId: string) {
    setSelectedGroupIds((current) => (current.includes(groupId) ? current.filter((id) => id !== groupId) : [...current, groupId]));
  }

  async function save() {
    if (!selectedProductId) return;
    setSaving(true);
    setError(null);
    try {
      await onSave(selectedProductId, selectedGroupIds);
      onClose();
    } catch {
      setError('No fue posible guardar la asignación.');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex h-[100dvh] items-end justify-center overflow-y-auto overscroll-contain bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="product-modifier-assignment-title">
      <div className="my-auto max-h-[calc(100dvh-1rem)] w-full max-w-lg space-y-4 overflow-y-auto rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl sm:max-h-[calc(100dvh-2rem)] sm:rounded-3xl sm:p-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">Modificadores</p>
            <h2 id="product-modifier-assignment-title" className="mt-1 text-2xl font-semibold text-white">Asignar a producto</h2>
          </div>
          <button type="button" onClick={onClose} className="text-2xl text-[var(--color-muted)]" aria-label="Cerrar asignación">×</button>
        </div>
        <label className="block text-sm text-[var(--color-muted)]">
          Producto
          <select value={selectedProductId} onChange={(event) => setSelectedProductId(event.target.value)} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white">
            {products.map((product) => <option key={product.id} value={product.id}>{product.name}</option>)}
          </select>
        </label>
        <fieldset className="space-y-2">
          <legend className="text-sm font-semibold text-white">Grupos de modificadores disponibles</legend>
          {loading ? <p className="text-sm text-[var(--color-muted)]">Cargando...</p> : modifierGroups.length ? modifierGroups.map((group) => (
            <label key={group.id} className="flex cursor-pointer items-center justify-between gap-3 rounded-xl border border-[var(--color-border)] p-3 text-sm text-[var(--color-muted)] has-[:checked]:border-[var(--color-primary)] has-[:checked]:text-white">
              <span className="flex items-center gap-2">
                <input type="checkbox" checked={selectedGroupIds.includes(group.id)} onChange={() => toggleGroup(group.id)} />
                {group.name}
              </span>
              <span className="text-xs">{group.required ? 'Obligatorio' : 'Opcional'}</span>
            </label>
          )) : <p className="text-sm text-[var(--color-muted)]">Aún no hay grupos de modificadores creados.</p>}
        </fieldset>
        {error ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</p> : null}
        <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end">
          <button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)]">Cancelar</button>
          <button type="button" onClick={() => void save()} disabled={!selectedProductId || saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Guardando...' : 'Guardar asignación'}</button>
        </div>
      </div>
    </div>
  );
}
