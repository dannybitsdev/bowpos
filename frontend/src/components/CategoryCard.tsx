import { useState } from 'react';

import type { MenuCategory } from '../pages/menuTypes';
import type { Product } from '../pages/menuTypes';

type CategoryCardProps = {
  category: MenuCategory;
  products: Product[];
  onEdit: (category: MenuCategory) => void;
  onDeactivate: (category: MenuCategory) => void;
  onDeleteProduct: (product: Product) => void;
};

export function CategoryCard({ category, products, onEdit, onDeactivate, onDeleteProduct }: CategoryCardProps) {
  const [showAllProducts, setShowAllProducts] = useState(false);
  const visibleProducts = showAllProducts ? products : products.slice(0, 6);
  const hasMoreProducts = products.length > 6;

  return (
    <article className="overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] shadow-panel">
      <div className="aspect-[16/9] bg-[#0D0D0D]">
        {category.image_url ? <img src={category.image_url} alt={category.name} loading="lazy" className="h-full w-full object-cover" /> : <div className="flex h-full items-center justify-center text-sm text-[var(--color-muted)]">Sin imagen</div>}
      </div>
      <div className="space-y-3 p-4">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0"><h2 className="break-words text-lg font-semibold text-white">{category.name}</h2><p className="mt-1 text-sm text-[var(--color-muted)]">{category.description || 'Sin descripción'}</p></div>
          <span className="shrink-0 rounded-full border border-[var(--color-border)] px-2 py-1 text-xs text-[var(--color-muted)]">Orden {category.display_order}</span>
        </div>
        <div className="rounded-xl border border-[var(--color-border)] bg-black/15 p-3">
          <div className="flex items-center justify-between gap-3"><p className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--color-muted)]">Productos asignados</p><span className="text-xs text-[var(--color-muted)]">{products.length}</span></div>
          {products.length ? <>
            <ul className="mt-2 space-y-2 text-sm text-white">{visibleProducts.map((product) => <li key={product.id} className="flex items-center justify-between gap-2"><span className="min-w-0 truncate">{product.name}</span><button type="button" onClick={() => onDeleteProduct(product)} className="shrink-0 text-xs text-rose-200 hover:text-rose-100" aria-label={`Eliminar ${product.name}`}>Eliminar</button></li>)}</ul>
            {hasMoreProducts ? <button type="button" onClick={() => setShowAllProducts((current) => !current)} className="mt-3 text-xs font-semibold text-[var(--color-primary)] hover:underline">{showAllProducts ? 'Mostrar menos' : `Ver todos (${products.length})`}</button> : null}
          </> : <p className="mt-2 text-sm text-[var(--color-muted)]">No hay productos asignados.</p>}
        </div>
        <div className="flex flex-wrap gap-2">
          <button type="button" onClick={() => onEdit(category)} className="rounded-lg border border-[var(--color-border)] px-3 py-2 text-sm text-white hover:border-[var(--color-primary)]">Editar</button>
          <button type="button" onClick={() => onDeactivate(category)} className="rounded-lg border border-rose-400/40 px-3 py-2 text-sm text-rose-200 hover:border-rose-300">Desactivar</button>
        </div>
      </div>
    </article>
  );
}
