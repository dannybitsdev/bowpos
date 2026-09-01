import { useEffect, useMemo, useState } from 'react';

import { CategoryCard } from '../components/CategoryCard';
import { CategoryFormModal } from '../components/CategoryFormModal';
import { ProductAssignModal } from '../components/ProductAssignModal';
import { useCategories } from '../hooks/useCategories';
import apiClient from '../features/auth/infrastructure/http/apiClient';
import { useConfirmationModal } from '../features/modal/presentation/useConfirmationModal';
import type { CategoryPayload, MenuCategory, Product } from './menuTypes';

export function CategoryManagementPage() {
  const { categories, loading, error, createCategory, updateCategory, deactivateCategory } = useCategories();
  const [products, setProducts] = useState<Product[]>([]);
  const [editingCategory, setEditingCategory] = useState<MenuCategory | null>(null);
  const [formOpen, setFormOpen] = useState(false);
  const [assignOpen, setAssignOpen] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const { confirm } = useConfirmationModal();

  async function loadProducts() {
    const response = await apiClient.get<Product[]>('/v1/menu/products');
    setProducts(response.data);
  }

  useEffect(() => { void loadProducts().catch(() => setActionError('No fue posible cargar los productos.')); }, []);

  const uncategorizedProducts = useMemo(() => products.filter((product) => !categories.some((category) => category.id === product.category_id)), [categories, products]);

  function openCreate() { setEditingCategory(null); setFormOpen(true); }
  function openEdit(category: MenuCategory) { setEditingCategory(category); setFormOpen(true); }

  async function saveCategory(payload: CategoryPayload) {
    setActionError(null);
    if (editingCategory) await updateCategory(editingCategory.id, payload);
    else await createCategory(payload);
  }

  async function confirmDeactivate(category: MenuCategory) {
    const confirmed = await confirm({
      title: 'Desactivar categoría',
      description: `¿Deseas desactivar la categoría "${category.name}"? Dejará de mostrarse en el menú.`,
      confirmLabel: 'Desactivar',
      cancelLabel: 'Cancelar',
      variant: 'warning',
    });
    if (!confirmed) return;
    try { await deactivateCategory(category.id); } catch { setActionError('No fue posible desactivar la categoría.'); }
  }

  async function assignProduct(product: Product, categoryId: string) {
    try {
      await apiClient.put(`/v1/menu/products/${product.id}`, { category_id: categoryId, name: product.name, description: product.description, price: product.price, stock: product.stock ?? 0, image_url: product.image_url });
      await loadProducts();
    } catch {
      setActionError('No fue posible asignar el producto a la categoría.');
      throw new Error('product assignment failed');
    }
  }

  async function deleteProduct(product: Product) {
    const confirmed = await confirm({
      title: 'Eliminar producto',
      description: `¿Deseas eliminar "${product.name}" del menú? Esta acción no se puede deshacer.`,
      confirmLabel: 'Eliminar',
      cancelLabel: 'Cancelar',
      variant: 'destructive',
    });
    if (!confirmed) return;
    setActionError(null);
    try {
      await apiClient.delete(`/v1/menu/products/${product.id}`);
      await loadProducts();
    } catch {
      setActionError('No fue posible eliminar el producto.');
    }
  }

  return <section className="min-h-screen min-w-0 bg-[var(--color-background)] p-4 text-[var(--color-text)] sm:p-5 lg:p-8">
    <header className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"><div><p className="text-[11px] font-semibold uppercase tracking-[0.35em] text-[var(--color-primary)]">Administración</p><h1 className="text-3xl font-semibold text-white sm:text-4xl">Categorías</h1><p className="mt-2 text-sm text-[var(--color-muted)]">Organiza el menú de tu establecimiento.</p></div><div className="flex flex-wrap gap-2"><button type="button" onClick={() => setAssignOpen(true)} disabled={!products.length || !categories.length} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-white disabled:opacity-40">Asignar productos</button><button type="button" onClick={openCreate} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black">Nueva categoría</button></div></header>
    {error || actionError ? <p className="mb-5 rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error ?? actionError}</p> : null}
    {loading ? <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">{[1, 2, 3].map((item) => <div key={item} className="h-72 animate-pulse rounded-2xl bg-[var(--color-card-bg)]" />)}</div> : categories.length ? <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">{categories.map((category) => <CategoryCard key={category.id} category={category} products={products.filter((product) => product.category_id === category.id)} onEdit={openEdit} onDeactivate={(item) => void confirmDeactivate(item)} onDeleteProduct={(product) => void deleteProduct(product)} />)}</div> : <div className="rounded-2xl border border-dashed border-[var(--color-border)] p-10 text-center text-sm text-[var(--color-muted)]">Aún no hay categorías activas.</div>}
    {formOpen ? <CategoryFormModal category={editingCategory} onClose={() => setFormOpen(false)} onSubmit={saveCategory} /> : null}
    {assignOpen ? <ProductAssignModal products={products} categories={categories} onAssign={assignProduct} onClose={() => setAssignOpen(false)} /> : null}
    {uncategorizedProducts.length ? <p className="mt-6 text-sm text-amber-200">Hay {uncategorizedProducts.length} productos sin categoría asignada.</p> : null}
  </section>;
}
