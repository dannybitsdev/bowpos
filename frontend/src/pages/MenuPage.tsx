import { useEffect, useMemo, useState, type FormEvent } from 'react';
import axios from 'axios';

import apiClient from '../features/auth/infrastructure/http/apiClient';
import { ProductCard } from '../components/ProductCard';
import type { MenuCategory, MenuResponse, Product, ProductPayload } from './menuTypes';
import { sortMenu } from './menuUtils';

const PRODUCTS_PER_PAGE = 24;

function MenuSkeleton() {
  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      {[1, 2, 3, 4, 5, 6].map((item) => <div key={item} className="h-72 animate-pulse rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)]" />)}
    </div>
  );
}

export function MenuPage() {
  const [categories, setCategories] = useState<MenuCategory[]>([]);
  const [selectedCategoryId, setSelectedCategoryId] = useState<string>('all');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isEditorOpen, setIsEditorOpen] = useState(false);
  const [editingProduct, setEditingProduct] = useState<Product | null>(null);
  const [saving, setSaving] = useState(false);
  const [deletingProductId, setDeletingProductId] = useState<string | null>(null);
  const [formError, setFormError] = useState<string | null>(null);
  const [page, setPage] = useState(1);

  async function loadMenu() {
    setLoading(true);
    try {
      const { data } = await apiClient.get<MenuResponse>('/v1/menu');
      setCategories(sortMenu(data.data));
      setError(null);
    } catch {
      setError('No fue posible cargar el menú.');
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadMenu();
  }, []);

  function openCreate() {
    setEditingProduct(null);
    setFormError(null);
    setIsEditorOpen(true);
  }

  function selectCategory(categoryId: string) {
    setSelectedCategoryId(categoryId);
    setPage(1);
  }

  function openEdit(product: Product) {
    setEditingProduct(product);
    setFormError(null);
    setIsEditorOpen(true);
  }

  async function saveProduct(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSaving(true);
    setFormError(null);
    const form = new FormData(event.currentTarget);
    const payload: ProductPayload = {
      category_id: String(form.get('category_id')),
      name: String(form.get('name')).trim(),
      description: String(form.get('description') ?? '').trim() || null,
      price: Number(form.get('price')),
      stock: Number(form.get('stock')),
      image_url: String(form.get('image_url') ?? '').trim() || null,
    };

    if (!payload.name || !payload.category_id || !Number.isFinite(payload.price) || payload.price < 0 || !Number.isInteger(payload.stock) || payload.stock < 0) {
      setFormError('Completa los campos obligatorios con valores válidos.');
      setSaving(false);
      return;
    }

    try {
      if (editingProduct) await apiClient.put(`/v1/menu/products/${editingProduct.id}`, payload);
      else await apiClient.post('/v1/menu/products', payload);
      setIsEditorOpen(false);
      await loadMenu();
    } catch (requestError) {
      if (axios.isAxiosError(requestError)) {
        const status = requestError.response?.status;
        const serverMessage = requestError.response?.data?.message;
        setFormError(serverMessage ?? (status === 401 ? 'Tu sesión expiró. Cierra sesión e inicia nuevamente.' : status === 403 ? 'No tienes permisos para editar productos.' : `No fue posible guardar el producto (error ${status ?? 'de conexión'}).`));
      } else {
        setFormError('No fue posible guardar el producto. Verifica los datos e inténtalo nuevamente.');
      }
    } finally {
      setSaving(false);
    }
  }

  async function deleteProduct(product: Product) {
    if (!window.confirm(`¿Eliminar "${product.name}" del menú?`)) return;

    setDeletingProductId(product.id);
    setError(null);
    try {
      await apiClient.delete(`/v1/menu/products/${product.id}`);
      await loadMenu();
    } catch (requestError) {
      if (axios.isAxiosError(requestError)) {
        const status = requestError.response?.status;
        setError(status === 401 ? 'Tu sesión expiró. Cierra sesión e inicia nuevamente.' : status === 403 ? 'No tienes permisos para eliminar productos.' : `No fue posible eliminar el producto (error ${status ?? 'de conexión'}).`);
      } else {
        setError('No fue posible eliminar el producto.');
      }
    } finally {
      setDeletingProductId(null);
    }
  }

  const selectedCategory = useMemo(
    () => categories.find((category) => category.id === selectedCategoryId),
    [categories, selectedCategoryId],
  );
  const filteredCategories = useMemo(
    () => selectedCategory ? [selectedCategory] : categories,
    [categories, selectedCategory],
  );
  const allProducts = useMemo(
    () => filteredCategories.flatMap((category) => category.products.map((product) => ({ product, category }))),
    [filteredCategories],
  );
  const totalPages = Math.max(1, Math.ceil(allProducts.length / PRODUCTS_PER_PAGE));
  const currentPage = Math.min(page, totalPages);
  const pagedProducts = allProducts.slice((currentPage - 1) * PRODUCTS_PER_PAGE, currentPage * PRODUCTS_PER_PAGE);
  const visibleCategories = filteredCategories
    .map((category) => ({ ...category, products: pagedProducts.filter((item) => item.category.id === category.id).map((item) => item.product) }))
    .filter((category) => category.products.length > 0);
  const firstVisibleProduct = allProducts.length === 0 ? 0 : (currentPage - 1) * PRODUCTS_PER_PAGE + 1;
  const lastVisibleProduct = Math.min(currentPage * PRODUCTS_PER_PAGE, allProducts.length);

  return (
    <section className="min-h-screen min-w-0 bg-[var(--color-background)] p-4 text-[var(--color-text)] sm:p-5 lg:p-8">
      <header className="mb-6 flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-[0.35em] text-[var(--color-primary)]">Catálogo</p>
          <h1 className="text-3xl font-semibold tracking-tight text-white sm:text-4xl">Menú</h1>
          <p className="mt-2 max-w-2xl text-sm leading-6 text-[var(--color-muted)]">Productos configurados para tu establecimiento, organizados por categoría.</p>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-sm text-[var(--color-muted)]">{categories.reduce((count, category) => count + category.products.length, 0)} productos</span>
          <button type="button" onClick={openCreate} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black transition hover:brightness-105">Agregar producto</button>
        </div>
      </header>

      {loading ? <MenuSkeleton /> : error ? (
        <div className="rounded-2xl border border-rose-500/30 bg-rose-500/10 p-5 text-sm text-rose-200">{error}</div>
      ) : categories.length === 0 ? (
        <div className="rounded-2xl border border-dashed border-[var(--color-border)] p-8 text-center text-sm text-[var(--color-muted)]">Aún no hay categorías activas en este tenant.</div>
      ) : (
        <>
          <div className="mb-8 overflow-x-auto pb-2" role="tablist" aria-label="Categorías del menú">
            <div className="flex min-w-max gap-2">
                  <button type="button" role="tab" aria-selected={selectedCategoryId === 'all'} onClick={() => selectCategory('all')} className={`rounded-full px-4 py-2 text-sm font-medium transition ${selectedCategoryId === 'all' ? 'bg-[var(--color-primary)] text-black' : 'border border-[var(--color-border)] text-[var(--color-muted)] hover:text-white'}`}>Todas</button>
                  {categories.map((category) => <button key={category.id} type="button" role="tab" aria-selected={selectedCategoryId === category.id} onClick={() => selectCategory(category.id)} className={`rounded-full px-4 py-2 text-sm font-medium transition ${selectedCategoryId === category.id ? 'bg-[var(--color-primary)] text-black' : 'border border-[var(--color-border)] text-[var(--color-muted)] hover:text-white'}`}>{category.name}</button>)}
            </div>
          </div>
          <div className="space-y-10">
            {visibleCategories.map((category) => <section key={category.id} aria-labelledby={`category-${category.id}`}>
              <div className="mb-4 flex items-end justify-between gap-3">
                <div className="min-w-0"><h2 id={`category-${category.id}`} className="break-words text-xl font-semibold text-white">{category.name}</h2>{category.description ? <p className="mt-1 text-sm text-[var(--color-muted)]">{category.description}</p> : null}</div>
                <span className="shrink-0 text-xs text-[var(--color-muted)]">{category.products.length} productos</span>
              </div>
              {category.products.length ? <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">{category.products.map((product) => <div key={product.id} className="relative"><ProductCard product={product} /><div className="absolute right-3 top-3 flex gap-2"><button type="button" onClick={() => openEdit(product)} className="rounded-lg border border-white/20 bg-black/75 px-2.5 py-1.5 text-xs font-medium text-white backdrop-blur transition hover:border-[var(--color-primary)] hover:text-[var(--color-primary)]">Editar</button><button type="button" onClick={() => deleteProduct(product)} disabled={deletingProductId === product.id} className="rounded-lg border border-rose-400/40 bg-black/75 px-2.5 py-1.5 text-xs font-medium text-rose-200 backdrop-blur transition hover:border-rose-300 hover:text-white disabled:opacity-50" aria-label={`Eliminar ${product.name}`}>{deletingProductId === product.id ? 'Eliminando...' : 'Eliminar'}</button></div></div>)}</div> : <p className="text-sm text-[var(--color-muted)]">No hay productos activos en esta categoría.</p>}
            </section>)}
          </div>
          {totalPages > 1 ? <nav className="mt-8 flex flex-col gap-3 border-t border-[var(--color-border)] pt-5 sm:flex-row sm:items-center sm:justify-between" aria-label="Paginación del menú">
            <p className="text-sm text-[var(--color-muted)]">Mostrando {firstVisibleProduct}-{lastVisibleProduct} de {allProducts.length} productos</p>
            <div className="flex items-center gap-2">
              <button type="button" onClick={() => setPage((value) => Math.max(1, value - 1))} disabled={currentPage === 1} className="rounded-xl border border-[var(--color-border)] px-3 py-2 text-sm text-[var(--color-muted)] transition hover:text-white disabled:cursor-not-allowed disabled:opacity-40">Anterior</button>
              <span className="min-w-20 text-center text-sm text-white">Página {currentPage} de {totalPages}</span>
              <button type="button" onClick={() => setPage((value) => Math.min(totalPages, value + 1))} disabled={currentPage === totalPages} className="rounded-xl border border-[var(--color-border)] px-3 py-2 text-sm text-[var(--color-muted)] transition hover:text-white disabled:cursor-not-allowed disabled:opacity-40">Siguiente</button>
            </div>
          </nav> : null}
        </>
      )}

      {isEditorOpen ? <div className="fixed inset-0 z-50 flex items-end justify-center bg-black/70 p-0 sm:items-center sm:p-4" role="dialog" aria-modal="true" aria-labelledby="product-editor-title">
        <form onSubmit={saveProduct} className="w-full max-w-xl space-y-4 rounded-t-3xl border border-[var(--color-border)] bg-[var(--color-card-bg)] p-5 shadow-2xl sm:rounded-3xl sm:p-6">
          <div className="flex items-start justify-between gap-4"><div><p className="text-[11px] font-semibold uppercase tracking-[0.3em] text-[var(--color-primary)]">Catálogo</p><h2 id="product-editor-title" className="mt-1 text-2xl font-semibold text-white">{editingProduct ? 'Editar producto' : 'Agregar producto'}</h2></div><button type="button" onClick={() => setIsEditorOpen(false)} className="text-2xl leading-none text-[var(--color-muted)] hover:text-white" aria-label="Cerrar formulario">×</button></div>
          <label className="block text-sm text-[var(--color-muted)]">Categoría<select name="category_id" defaultValue={editingProduct?.category_id ?? categories[0]?.id ?? ''} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]">{categories.map((category) => <option key={category.id} value={category.id}>{category.name}</option>)}</select></label>
          <label className="block text-sm text-[var(--color-muted)]">Nombre<input name="name" defaultValue={editingProduct?.name ?? ''} required maxLength={255} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label>
          <label className="block text-sm text-[var(--color-muted)]">Descripción<textarea name="description" defaultValue={editingProduct?.description ?? ''} rows={3} className="mt-2 w-full resize-y rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label>
          <div className="grid gap-4 sm:grid-cols-2"><label className="block text-sm text-[var(--color-muted)]">Precio<input name="price" type="number" min="0" step="0.01" defaultValue={editingProduct?.price ?? 0} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label><label className="block text-sm text-[var(--color-muted)]">Existencias<input name="stock" type="number" min="0" step="1" defaultValue={editingProduct?.stock ?? 0} required className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /></label></div>
          <label className="block text-sm text-[var(--color-muted)]">URL directa de imagen<input name="image_url" type="url" placeholder="https://ejemplo.com/imagen.jpg" defaultValue={editingProduct?.image_url ?? ''} className="mt-2 w-full rounded-xl border border-[var(--color-border)] bg-[#0D0D0D] px-3 py-2.5 text-white outline-none focus:border-[var(--color-primary)]" /><span className="mt-1 block text-xs text-[var(--color-muted)]">Usa una URL que termine en una imagen o entregue contenido JPG, PNG o WebP; no una página de galería.</span></label>
          {formError ? <p className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{formError}</p> : null}
          <div className="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end"><button type="button" onClick={() => setIsEditorOpen(false)} className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-sm text-[var(--color-muted)] hover:text-white">Cancelar</button><button type="submit" disabled={saving} className="rounded-xl bg-[var(--color-primary)] px-4 py-2.5 text-sm font-semibold text-black disabled:opacity-60">{saving ? 'Guardando...' : 'Guardar producto'}</button></div>
        </form>
      </div> : null}
    </section>
  );
}
