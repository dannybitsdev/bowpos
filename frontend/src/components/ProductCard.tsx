import { useState } from 'react';

import type { Product } from '../pages/menuTypes';

type ProductCardProps = {
  product: Product;
};

export function ProductCard({ product }: ProductCardProps) {
  const [imageFailed, setImageFailed] = useState(false);

  return (
    <article className="flex min-w-0 flex-col overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] shadow-panel transition hover:border-[var(--color-primary)]/60">
      <div className="aspect-[4/3] overflow-hidden bg-[#0D0D0D]">
        {product.image_url && !imageFailed ? (
          <img src={product.image_url} alt={product.name} loading="lazy" onError={() => setImageFailed(true)} className="h-full w-full object-cover" />
        ) : (
          <div className="flex h-full flex-col items-center justify-center gap-2 text-sm text-[var(--color-muted)]"><span className="text-2xl" aria-hidden="true">🍽</span><span>{product.image_url ? 'Imagen no disponible' : 'Sin imagen'}</span></div>
        )}
      </div>
      <div className="flex flex-1 flex-col p-4">
        <div className="flex items-start justify-between gap-3">
          <h3 className="min-w-0 break-words text-base font-semibold text-white">{product.name}</h3>
          <span className="shrink-0 rounded-full bg-[var(--color-primary)] px-2.5 py-1 text-xs font-semibold text-black">
            {new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(product.price)}
          </span>
        </div>
        {product.description ? <p className="mt-3 line-clamp-3 text-sm leading-6 text-[var(--color-muted)]">{product.description}</p> : null}
      </div>
    </article>
  );
}