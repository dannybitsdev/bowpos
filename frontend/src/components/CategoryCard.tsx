import type { MenuCategory } from '../pages/menuTypes';

type CategoryCardProps = {
  category: MenuCategory;
  onEdit: (category: MenuCategory) => void;
  onDeactivate: (category: MenuCategory) => void;
};

export function CategoryCard({ category, onEdit, onDeactivate }: CategoryCardProps) {
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
        <div className="flex flex-wrap gap-2">
          <button type="button" onClick={() => onEdit(category)} className="rounded-lg border border-[var(--color-border)] px-3 py-2 text-sm text-white hover:border-[var(--color-primary)]">Editar</button>
          <button type="button" onClick={() => onDeactivate(category)} className="rounded-lg border border-rose-400/40 px-3 py-2 text-sm text-rose-200 hover:border-rose-300">Desactivar</button>
        </div>
      </div>
    </article>
  );
}
