import { useEffect, useState } from 'react';

import apiClient from '../features/auth/infrastructure/http/apiClient';
import { useConfirmationModal } from '../features/modal/presentation/useConfirmationModal';
import { useProductModifiers } from '../hooks/useProductModifiers';
import { ModifierGroupCard } from '../components/ModifierGroupCard';
import { ModifierGroupFormModal } from '../components/ModifierGroupFormModal';
import { ModifierFormModal } from '../components/ModifierFormModal';
import { ProductModifierAssignModal } from '../components/ProductModifierAssignModal';
import type { Modifier, ModifierGroup, Product } from './menuTypes';

export function ModifierGroupsPage() {
  const {
    modifierGroups,
    loading,
    error,
    createModifierGroup,
    updateModifierGroup,
    deactivateModifierGroup,
    createModifier,
    updateModifier,
    deleteModifier,
    getProductModifierGroupIds,
    setProductModifierGroups,
  } = useProductModifiers();
  const { confirm } = useConfirmationModal();

  const [products, setProducts] = useState<Product[]>([]);
  const [groupFormOpen, setGroupFormOpen] = useState(false);
  const [editingGroup, setEditingGroup] = useState<ModifierGroup | null>(null);
  const [modifierFormGroup, setModifierFormGroup] = useState<ModifierGroup | null>(null);
  const [editingModifier, setEditingModifier] = useState<Modifier | null>(null);
  const [assignOpen, setAssignOpen] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => {
    void apiClient.get<Product[]>('/v1/menu/products').then((response) => setProducts(response.data)).catch(() => setActionError('No fue posible cargar los productos.'));
  }, []);

  function openCreateGroup() { setEditingGroup(null); setGroupFormOpen(true); }
  function openEditGroup(group: ModifierGroup) { setEditingGroup(group); setGroupFormOpen(true); }

  async function handleDeactivateGroup(group: ModifierGroup) {
    const confirmed = await confirm({
      title: 'Desactivar grupo de modificadores',
      description: `¿Deseas desactivar "${group.name}"? Dejará de estar disponible para nuevos productos.`,
      confirmLabel: 'Desactivar',
      cancelLabel: 'Cancelar',
      variant: 'warning',
    });
    if (!confirmed) return;
    setActionError(null);
    try { await deactivateModifierGroup(group.id); } catch { setActionError('No fue posible desactivar el grupo.'); }
  }

  async function handleDeleteModifier(modifier: Modifier) {
    const confirmed = await confirm({
      title: 'Eliminar modificador',
      description: `¿Deseas eliminar "${modifier.name}"? Esta acción no se puede deshacer.`,
      confirmLabel: 'Eliminar',
      cancelLabel: 'Cancelar',
      variant: 'destructive',
    });
    if (!confirmed) return;
    setActionError(null);
    try { await deleteModifier(modifier.id); } catch { setActionError('No fue posible eliminar el modificador.'); }
  }

  return (
    <section className="min-h-screen min-w-0 bg-[var(--color-background)] p-4 text-[var(--color-text)] sm:p-5 lg:p-8">
      <header className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-[0.35em] text-[var(--color-primary)]">Administración</p>
          <h1 className="text-3xl font-semibold text-white sm:text-4xl">Modificadores</h1>
          <p className="mt-2 text-sm text-[var(--color-muted)]">Crea adicionales y opciones de personalización para tus productos.</p>
        </div>
        <div className="flex flex-wrap gap-2">
          <button type="button" onClick={() => setAssignOpen(true)} disabled={!products.length || !modifierGroups.length} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-white disabled:opacity-40">Asignar a producto</button>
          <button type="button" onClick={openCreateGroup} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black">Nuevo grupo</button>
        </div>
      </header>

      {error || actionError ? <p className="mb-5 rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error ?? actionError}</p> : null}

      {loading ? (
        <div className="grid gap-4 grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
          {[1, 2, 3].map((item) => <div key={item} className="h-64 animate-pulse rounded-2xl bg-[var(--color-card-bg)]" />)}
        </div>
      ) : modifierGroups.length ? (
        <div className="grid gap-4 grid-cols-1 sm:grid-cols-2 xl:grid-cols-3">
          {modifierGroups.map((group) => (
            <ModifierGroupCard
              key={group.id}
              group={group}
              onEditGroup={openEditGroup}
              onDeactivateGroup={(item) => void handleDeactivateGroup(item)}
              onAddModifier={(item) => { setModifierFormGroup(item); setEditingModifier(null); }}
              onEditModifier={(groupItem, modifier) => { setModifierFormGroup(groupItem); setEditingModifier(modifier); }}
              onDeleteModifier={(modifier) => void handleDeleteModifier(modifier)}
            />
          ))}
        </div>
      ) : (
        <div className="rounded-2xl border border-dashed border-[var(--color-border)] p-10 text-center text-sm text-[var(--color-muted)]">Aún no hay grupos de modificadores.</div>
      )}

      {groupFormOpen ? (
        <ModifierGroupFormModal
          group={editingGroup}
          onClose={() => setGroupFormOpen(false)}
          onSubmit={(payload) => (editingGroup ? updateModifierGroup(editingGroup.id, payload) : createModifierGroup(payload))}
        />
      ) : null}

      {modifierFormGroup ? (
        <ModifierFormModal
          modifier={editingModifier}
          groupName={modifierFormGroup.name}
          onClose={() => { setModifierFormGroup(null); setEditingModifier(null); }}
          onSubmit={(payload) => (editingModifier ? updateModifier(editingModifier.id, payload) : createModifier(modifierFormGroup.id, payload))}
        />
      ) : null}

      {assignOpen ? (
        <ProductModifierAssignModal
          products={products}
          modifierGroups={modifierGroups}
          onLoadAssignedGroupIds={getProductModifierGroupIds}
          onSave={setProductModifierGroups}
          onClose={() => setAssignOpen(false)}
        />
      ) : null}
    </section>
  );
}
