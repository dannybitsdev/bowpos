import { useState } from 'react';

import type { MenuCategory, Product } from '../pages/menuTypes';

type ProductAssignModalProps = {
  products: Product[];
  categories: MenuCategory[];
  onAssign: (product: Product, categoryId: string) => Promise<void>;
  onClose: () => void;
};

export function ProductAssignModal({ products, categories, onAssign, onClose }: ProductAssignModalProps) {
  const [selectedProductId, setSelectedProductId] = useState(products[0]?.id ?? '');
  const [categoryId, setCategoryId] = useState(categories[0]?.id ?? '');
  const [saving, setSaving] = useState(false);
  const product = products.find((item) => item.id === selectedProductId);

  async function assignProduct() {
    if (!product || !categoryId) return;
    setSaving(true);
    try { await onAssign(product, categoryId); onClose(); } finally { setSaving(false); }
  }

  return <div className="fixed inset-0 z-50 flex items-end justify-center bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="product-assignment-title">
    <div className="w-full max-w-lg space-y-4 rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-2xl sm:rounded-3xl sm:p-6">
      <div className="flex items-start justify-between gap-4"><div><p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">Organización</p><h2 id="product-assignment-title" className="mt-1 text-2xl font-semibold text-white">Asignar productos</h2></div><button type="button" onClick={onClose} className="text-2xl text-[var(--color-muted)]" aria-label="Cerrar asignación">×</button></div>
      <label className="block text-sm text-[var(--color-muted)]">Producto<select value={selectedProductId} onChange={(event) => setSelectedProductId(event.target.value)} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white">{products.map((item) => <option key={item.id} value={item.id}>{item.name}</option>)}</select></label>
      <label className="block text-sm text-[var(--color-muted)]">Categoría destino<select value={categoryId} onChange={(event) => setCategoryId(event.target.value)} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white">{categories.map((category) => <option key={category.id} value={category.id}>{category.name}</option>)}</select></label>
      <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end"><button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)]">Cancelar</button><button type="button" onClick={() => void assignProduct()} disabled={!product || saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Asignando...' : 'Asignar producto'}</button></div>
    </div>
  </div>;
}
