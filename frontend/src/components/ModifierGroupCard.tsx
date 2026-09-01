import type { Modifier, ModifierGroup } from '../pages/menuTypes';

const currency = (value: number) => new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(value);

type ModifierGroupCardProps = {
  group: ModifierGroup;
  onEditGroup: (group: ModifierGroup) => void;
  onDeactivateGroup: (group: ModifierGroup) => void;
  onAddModifier: (group: ModifierGroup) => void;
  onEditModifier: (group: ModifierGroup, modifier: Modifier) => void;
  onDeleteModifier: (modifier: Modifier) => void;
};

export function ModifierGroupCard({ group, onEditGroup, onDeactivateGroup, onAddModifier, onEditModifier, onDeleteModifier }: ModifierGroupCardProps) {
  return (
    <article className="flex h-full flex-col rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-4 shadow-panel">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <h3 className="break-words text-lg font-semibold text-white">{group.name}</h3>
          <p className="mt-1 text-xs text-[var(--color-muted)]">
            {group.required ? 'Obligatorio' : 'Opcional'} · Selecciona {group.min_selections}-{group.max_selections}
          </p>
        </div>
        <span className="shrink-0 rounded-full border border-[var(--color-border)] px-2 py-1 text-xs text-[var(--color-muted)]">{group.modifiers.length} opciones</span>
      </div>

      <ul className="mt-4 min-h-0 flex-1 space-y-2">
        {group.modifiers.length ? group.modifiers.map((modifier) => (
          <li key={modifier.id} className="flex items-center justify-between gap-3 rounded-xl border border-[var(--color-border)] p-3 text-sm">
            <div className="min-w-0">
              <p className={`truncate font-medium ${modifier.is_active ? 'text-white' : 'text-[var(--color-muted)] line-through'}`}>{modifier.name}</p>
              <p className="text-xs text-[var(--color-muted)]">{modifier.price_delta > 0 ? `+${currency(modifier.price_delta)}` : 'Sin costo adicional'}</p>
            </div>
            <div className="flex shrink-0 gap-1">
              <button type="button" onClick={() => onEditModifier(group, modifier)} className="flex min-h-[44px] min-w-[44px] items-center justify-center rounded-lg px-2 text-xs font-medium text-[var(--color-primary)]">Editar</button>
              <button type="button" onClick={() => onDeleteModifier(modifier)} className="flex min-h-[44px] min-w-[44px] items-center justify-center rounded-lg px-2 text-xs font-medium text-rose-200">Eliminar</button>
            </div>
          </li>
        )) : <li className="text-sm text-[var(--color-muted)]">Este grupo aún no tiene modificadores.</li>}
      </ul>

      <div className="mt-4 flex flex-wrap gap-2 border-t border-[var(--color-border)] pt-4">
        <button type="button" onClick={() => onAddModifier(group)} className="min-h-[44px] rounded-lg bg-[var(--color-primary)] px-3 py-2 text-sm font-semibold text-black">Agregar modificador</button>
        <button type="button" onClick={() => onEditGroup(group)} className="min-h-[44px] rounded-lg border border-[var(--color-border)] px-3 py-2 text-sm text-white hover:border-[var(--color-primary)]">Editar grupo</button>
        <button type="button" onClick={() => onDeactivateGroup(group)} className="min-h-[44px] rounded-lg border border-rose-400/40 px-3 py-2 text-sm text-rose-200 hover:border-rose-300">Desactivar</button>
      </div>
    </article>
  );
}
