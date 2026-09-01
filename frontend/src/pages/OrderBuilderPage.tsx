import { useEffect, useMemo, useState } from 'react';
import axios from 'axios';

import apiClient from '../features/auth/infrastructure/http/apiClient';
import { OrderSummary } from '../components/OrderSummary';
import { ProductCustomizeModal } from '../components/ProductCustomizeModal';
import type { CatalogProduct, CreateOrderPayload, OrderDraftItem, ServiceType } from './orderTypes';

const currency = (value: number) => new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(value);

export function OrderBuilderPage() {
  const [catalog, setCatalog] = useState<CatalogProduct[]>([]);
  const [selectedProduct, setSelectedProduct] = useState<CatalogProduct | null>(null);
  const [editingItem, setEditingItem] = useState<OrderDraftItem | undefined>();
  const [items, setItems] = useState<OrderDraftItem[]>([]);
  const [serviceType, setServiceType] = useState<ServiceType>('DINE_IN');
  const [tableName, setTableName] = useState('');
  const [customerName, setCustomerName] = useState('');
  const [orderNotes, setOrderNotes] = useState('');
  const [search, setSearch] = useState('');
  const [activeTab, setActiveTab] = useState<'catalog' | 'summary'>('catalog');
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  useEffect(() => {
    void apiClient.get<CatalogProduct[]>('/v1/orders/catalog')
      .then((response) => setCatalog(response.data))
      .catch(() => setMessage('No fue posible cargar el catálogo.'));
  }, []);

  const filteredCatalog = useMemo(
    () => catalog.filter((product) => product.name.toLocaleLowerCase().includes(search.toLocaleLowerCase())),
    [catalog, search],
  );

  function saveItem(item: OrderDraftItem) {
    setItems((current) => editingItem ? current.map((value) => value.id === item.id ? item : value) : [...current, item]);
    setSelectedProduct(null);
    setEditingItem(undefined);
    setActiveTab('summary');
  }

  function editItem(item: OrderDraftItem) {
    setEditingItem(item);
    setSelectedProduct(item.product);
  }

  async function processOrder() {
    setSaving(true);
    setMessage(null);
    const payload: CreateOrderPayload = {
      service_type: serviceType,
      table_name: tableName.trim() || undefined,
      customer_name: customerName.trim() || undefined,
      notes: orderNotes.trim() || undefined,
      tax_rate: 0.19,
      tip: 0,
      discount: 0,
      items: items.map((item) => ({ product_id: item.product.id, quantity: item.quantity, notes: item.notes || undefined, modifier_ids: item.modifierIds, topping_ids: item.toppingIds })),
    };

    try {
      await apiClient.post('/v1/orders', payload);
      setItems([]);
      setTableName('');
      setCustomerName('');
      setOrderNotes('');
      setMessage('Orden creada correctamente.');
      setActiveTab('catalog');
    } catch (requestError) {
      const serverMessage = axios.isAxiosError(requestError) ? requestError.response?.data?.message : null;
      setMessage(serverMessage ?? 'No fue posible procesar la orden. Revisa los productos e inténtalo nuevamente.');
    } finally {
      setSaving(false);
    }
  }

  return (
    <section className="min-h-screen min-w-0 bg-[var(--color-background)] p-4 text-[var(--color-text)] sm:p-5 lg:p-8">
      <header className="mb-5 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-[0.35em] text-[var(--color-primary)]">Operación</p>
          <h1 className="text-3xl font-semibold text-white sm:text-4xl">Nueva orden</h1>
          <p className="mt-2 text-sm text-[var(--color-muted)]">Personaliza cada producto y envía el pedido a preparación.</p>
        </div>
        <a href="/orders/history" className="rounded-xl border border-[var(--color-border)] px-4 py-2.5 text-center text-sm text-white hover:border-[var(--color-primary)]">Historial de órdenes</a>
        <div className="flex rounded-xl border border-[var(--color-border)] p-1 lg:hidden">
          <button type="button" onClick={() => setActiveTab('catalog')} className={`flex-1 rounded-lg px-4 py-2 text-sm ${activeTab === 'catalog' ? 'bg-[var(--color-primary)] font-semibold text-black' : 'text-[var(--color-muted)]'}`}>Catálogo</button>
          <button type="button" onClick={() => setActiveTab('summary')} className={`flex-1 rounded-lg px-4 py-2 text-sm ${activeTab === 'summary' ? 'bg-[var(--color-primary)] font-semibold text-black' : 'text-[var(--color-muted)]'}`}>Pedido ({items.length})</button>
        </div>
      </header>
      {message ? <p className="mb-5 rounded-xl border border-[var(--color-primary)]/30 bg-[var(--color-primary)]/10 p-3 text-sm text-[var(--color-primary)]">{message}</p> : null}
      <div className="grid gap-5 lg:grid-cols-[minmax(0,1fr)_minmax(20rem,26rem)]">
        <div className={activeTab === 'summary' ? 'hidden lg:block' : ''}>
          <div className="mb-4 flex flex-col gap-3 sm:flex-row">
            <input value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Buscar producto..." className="min-w-0 flex-1 rounded-xl border border-[var(--color-border)] bg-[var(--color-card-bg)] px-4 py-3 text-sm text-white outline-none focus:border-[var(--color-primary)]" />
            <div className="flex gap-2"><input value={tableName} onChange={(event) => setTableName(event.target.value)} placeholder="Mesa" className="w-28 rounded-xl border border-[var(--color-border)] bg-[var(--color-card-bg)] px-3 py-3 text-sm text-white outline-none focus:border-[var(--color-primary)]" /><input value={customerName} onChange={(event) => setCustomerName(event.target.value)} placeholder="Cliente" className="w-32 rounded-xl border border-[var(--color-border)] bg-[var(--color-card-bg)] px-3 py-3 text-sm text-white outline-none focus:border-[var(--color-primary)]" /></div>
          </div>
          <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
            {filteredCatalog.map((product) => <button type="button" key={product.id} onClick={() => { setEditingItem(undefined); setSelectedProduct(product); }} className="flex min-w-0 flex-col overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-card-bg)] text-left transition hover:border-[var(--color-primary)]/70"><div className="aspect-[4/3] overflow-hidden bg-[#0D0D0D]">{product.image_url ? <img src={product.image_url} alt={product.name} loading="lazy" className="h-full w-full object-cover" /> : <div className="flex h-full items-center justify-center text-3xl" aria-hidden="true">🍽</div>}</div><div className="flex items-center justify-between gap-3 p-4"><span className="min-w-0 break-words font-semibold text-white">{product.name}</span><span className="shrink-0 text-sm text-[var(--color-primary)]">{currency(product.price)}</span></div></button>)}
          </div>
        </div>
        <div className={activeTab === 'catalog' ? 'hidden lg:block' : ''}>
          <OrderSummary items={items} serviceType={serviceType} orderNotes={orderNotes} onOrderNotesChange={setOrderNotes} onServiceTypeChange={setServiceType} onEdit={editItem} onRemove={(id) => setItems((current) => current.filter((item) => item.id !== id))} onSubmit={() => void processOrder()} saving={saving} />
        </div>
      </div>
      {selectedProduct ? <ProductCustomizeModal product={selectedProduct} initialItem={editingItem} onClose={() => { setSelectedProduct(null); setEditingItem(undefined); }} onSave={saveItem} /> : null}
    </section>
  );
}
