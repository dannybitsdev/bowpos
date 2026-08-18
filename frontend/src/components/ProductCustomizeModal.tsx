import { useMemo, useState } from 'react';
import type { CatalogProduct, OrderDraftItem } from '../pages/orderTypes';

type Props = { product: CatalogProduct; initialItem?: OrderDraftItem; onClose: () => void; onSave: (item: OrderDraftItem) => void };
const currency = (value: number) => new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(value);

export function ProductCustomizeModal({ product, initialItem, onClose, onSave }: Props) {
  const [modifierIds, setModifierIds] = useState(initialItem?.modifierIds ?? []);
  const [toppingIds, setToppingIds] = useState(initialItem?.toppingIds ?? []);
  const [quantity, setQuantity] = useState(initialItem?.quantity ?? 1);
  const [notes, setNotes] = useState(initialItem?.notes ?? '');
  const [error, setError] = useState<string | null>(null);
  const selectedOptions = useMemo(() => [...product.modifier_groups.flatMap((group) => group.modifiers), ...product.toppings].filter((option) => modifierIds.includes(option.id) || toppingIds.includes(option.id)), [modifierIds, toppingIds, product]);
  const unitPrice = product.price + selectedOptions.reduce((sum, option) => sum + option.price, 0);

  function chooseModifier(groupId: string, optionId: string) {
    const group = product.modifier_groups.find((item) => item.id === groupId);
    if (!group) return;
    setModifierIds((current) => [...current.filter((id) => !group.modifiers.some((option) => option.id === id)), optionId]);
  }
  function toggleTopping(id: string) { setToppingIds((current) => current.includes(id) ? current.filter((value) => value !== id) : [...current, id]); }
  function save() {
    const invalid = product.modifier_groups.find((group) => modifierIds.filter((id) => group.modifiers.some((option) => option.id === id)).length < group.min_selections);
    if (invalid) { setError(`Selecciona una opción en ${invalid.name}.`); return; }
    onSave({ id: initialItem?.id ?? crypto.randomUUID(), product, quantity, modifierIds, toppingIds, notes: notes.trim() });
  }

  return <div className="fixed inset-0 z-50 flex h-[100dvh] items-end justify-center overflow-y-auto bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="customize-product-title">
    <div className="my-auto max-h-[calc(100dvh-1rem)] w-full max-w-2xl overflow-y-auto rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl sm:max-h-[calc(100dvh-2rem)] sm:rounded-3xl sm:p-6">
      <header className="flex items-start justify-between gap-4"><div><p className="text-[11px] font-semibold uppercase tracking-[0.25em] text-[var(--color-primary)]">Personalizar</p><h2 id="customize-product-title" className="mt-1 text-2xl font-semibold text-white">{product.name}</h2></div><button type="button" onClick={onClose} aria-label="Cerrar personalización" className="text-2xl text-[var(--color-muted)]">×</button></header>
      <div className="mt-6 space-y-6">
        {product.modifier_groups.map((group) => <fieldset key={group.id}><legend className="text-sm font-semibold text-white">{group.name}{group.required ? <span className="ml-2 text-xs font-normal text-[var(--color-primary)]">Obligatorio</span> : null}</legend><div className="mt-2 grid gap-2 sm:grid-cols-2">{group.modifiers.map((option) => <label key={option.id} className="flex cursor-pointer items-center justify-between gap-3 rounded-xl border border-[var(--color-border)] p-3 text-sm text-[var(--color-muted)] has-[:checked]:border-[var(--color-primary)] has-[:checked]:text-white"><span className="flex items-center gap-2"><input type="radio" name={group.id} checked={modifierIds.includes(option.id)} onChange={() => chooseModifier(group.id, option.id)} />{option.name}</span>{option.price ? <span>{currency(option.price)}</span> : null}</label>)}</div></fieldset>)}
        {product.toppings.length ? <fieldset><legend className="text-sm font-semibold text-white">Adicionales</legend><div className="mt-2 grid gap-2 sm:grid-cols-2">{product.toppings.map((option) => <label key={option.id} className="flex cursor-pointer items-center justify-between gap-3 rounded-xl border border-[var(--color-border)] p-3 text-sm text-[var(--color-muted)] has-[:checked]:border-[var(--color-primary)] has-[:checked]:text-white"><span className="flex items-center gap-2"><input type="checkbox" checked={toppingIds.includes(option.id)} onChange={() => toggleTopping(option.id)} />{option.name}</span><span>{currency(option.price)}</span></label>)}</div></fieldset> : null}
        <label className="block text-sm text-[var(--color-muted)]">Notas<textarea value={notes} onChange={(event) => setNotes(event.target.value)} rows={3} placeholder="Sin cebolla, salsa aparte..." className="mt-2 w-full resize-y rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label>
        <div className="flex items-center justify-between gap-4"><label className="text-sm text-[var(--color-muted)]">Cantidad<input type="number" min="1" value={quantity} onChange={(event) => setQuantity(Math.max(1, Number(event.target.value)))} className="ml-3 w-20 rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2 text-center text-white" /></label><p className="text-right text-lg font-semibold text-white">{currency(unitPrice * quantity)}</p></div>
        {error ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</p> : null}
        <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end"><button type="button" onClick={onClose} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)]">Cancelar</button><button type="button" onClick={save} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black">Agregar al pedido</button></div>
      </div>
    </div>
  </div>;
}
