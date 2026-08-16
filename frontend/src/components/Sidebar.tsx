import React from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import { useAuthStore } from '../features/auth/application/authStore';

type NavItem = {
  label: string;
  active?: boolean;
  icon: React.ReactNode;
};

const navItems: NavItem[] = [
  { label: 'Dashboard', active: true, icon: <DashboardIcon /> },
  { label: 'Ventas', icon: <SalesIcon /> },
  { label: 'Órdenes', icon: <OrdersIcon /> },
  { label: 'Menú', icon: <MenuIcon /> },
  { label: 'Inventarios', icon: <InventoryIcon /> },
  { label: 'Pagos', icon: <PaymentsIcon /> },
  { label: 'Facturas', icon: <InvoiceIcon /> },
  { label: 'Clientes', icon: <ClientsIcon /> },
  { label: 'Empleados', icon: <UsersIcon /> },
  { label: 'Cumplimiento Legal', icon: <ComplianceIcon /> },
  { label: 'Marketing con IA', icon: <MarketingIcon /> },
  { label: 'Reportes', icon: <ReportsIcon /> },
  { label: 'Configuración', icon: <SettingsIcon /> },
];

function DashboardIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M4 13.5 12 5l8 8.5V20a1 1 0 0 1-1 1h-4v-5H9v5H5a1 1 0 0 1-1-1z" />
    </svg>
  );
}

function SalesIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M5 18V9m7 9V5m7 13v-7" />
    </svg>
  );
}

function OrdersIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <rect x="4" y="5" width="16" height="14" rx="2" />
      <path d="M8 10h8M8 14h5" />
    </svg>
  );
}

function MenuIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M5 7h14M5 12h14M5 17h10" />
    </svg>
  );
}

function InventoryIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M5 7.5 12 4l7 3.5v9L12 20l-7-3.5z" />
      <path d="M12 8v8" />
    </svg>
  );
}

function PaymentsIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <rect x="3" y="6" width="18" height="12" rx="2" />
      <path d="M3 10h18" />
    </svg>
  );
}

function InvoiceIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M7 3h8l4 4v14H7z" />
      <path d="M15 3v5h5" />
    </svg>
  );
}

function ClientsIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M8 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm8 0a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm-8 2c-2.5 0-4.5 1.6-4.5 3.5V18h9v-1.5C12.5 14.6 10.5 13 8 13Zm8 0c-1.7 0-3.2 1-3.9 2.4" />
    </svg>
  );
}

function UsersIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M16 19a4 4 0 0 0-8 0M12 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
      <path d="M19 19a3 3 0 0 0-2.3-2.9M5 16.1A3 3 0 0 1 7.3 16" />
    </svg>
  );
}

function ComplianceIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="m5 12 4 4 10-10" />
    </svg>
  );
}

function MarketingIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M4 19h16M7 15l2.5-4 3 2 4.5-6" />
    </svg>
  );
}

function ReportsIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <path d="M6 4h10l2 2v14H6z" />
      <path d="M9 11h6M9 15h4" />
    </svg>
  );
}

function SettingsIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8">
      <circle cx="12" cy="12" r="3" />
      <path d="M19 12a7 7 0 0 0-.1-1.1l2.1-1.6-2-3.5-2.6 1a7.4 7.4 0 0 0-1.9-1.1L14.4 2h-4.8l-.8 2.8a7.4 7.4 0 0 0-1.9 1.1l-2.6-1-2 3.5 2.1 1.6A7 7 0 0 0 5 12c0 .4 0 .7.1 1.1L3 14.7l2 3.5 2.6-1a7.4 7.4 0 0 0 1.9 1.1l.8 2.8h4.8l.8-2.8a7.4 7.4 0 0 0 1.9-1.1l2.6 1 2-3.5-2.1-1.6c.1-.4.1-.8.1-1.1Z" />
    </svg>
  );
}

type SidebarProps = {
  open?: boolean;
  onClose?: () => void;
};

export const Sidebar: React.FC<SidebarProps> = ({ open = false, onClose }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const tenantName = useAuthStore((state) => state.user?.tenant_name ?? 'Bits TI Tecnología');

  return (
    <>
      {open ? <button type="button" aria-label="Cerrar navegación" onClick={onClose} className="fixed inset-0 z-30 bg-black/70 lg:hidden" /> : null}
      <aside className={`fixed inset-y-0 left-0 z-40 flex w-[min(18rem,calc(100vw-2rem))] flex-col border-r border-[var(--color-border)] bg-[var(--color-sidebar-bg)] px-5 py-6 text-[var(--color-text)] shadow-2xl transition-transform duration-200 lg:sticky lg:top-0 lg:z-10 lg:h-screen lg:shrink-0 lg:translate-x-0 lg:shadow-none ${open ? 'translate-x-0' : '-translate-x-full'}`}>
      <div className="mb-8 flex items-start justify-between gap-3">
        <div className="flex min-w-0 items-center gap-3">
        <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-[var(--color-primary)] text-lg font-black text-black shadow-lg shadow-black/20">
          C
        </div>
        <div className="min-w-0">
          <p className="break-words text-lg font-semibold tracking-tight">{tenantName}</p>
          <p className="text-sm text-[var(--color-muted)]">Sistema POS</p>
        </div>
        </div>
        <button type="button" onClick={onClose} className="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl border border-[var(--color-border)] text-[var(--color-muted)] hover:text-[var(--color-text)] lg:hidden" aria-label="Cerrar navegación">
          <span className="sr-only">Cerrar navegación</span>
          <svg viewBox="0 0 24 24" className="h-5 w-5" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden="true">
            <path d="m6 6 12 12M18 6 6 18" />
          </svg>
        </button>
      </div>

      <label className="mb-6 block text-sm text-[var(--color-muted)]">
        <span className="mb-2 block text-[11px] uppercase tracking-[0.3em]">Sedes</span>
        <select className="w-full rounded-xl border border-[var(--color-border)] bg-[#101010] px-3 py-2.5 text-sm text-[var(--color-text)] outline-none transition focus:border-[var(--color-primary)]">
          <option>Sede Principal</option>
          <option>Sede Norte</option>
          <option>Sede Sur</option>
        </select>
      </label>

      <nav className="min-h-0 flex-1 space-y-1.5 overflow-y-auto pr-1">
        {navItems.map((item) => {
          const isMenu = item.label === 'Menú';
          const isActive = isMenu ? location.pathname === '/menu' : location.pathname === '/dashboard';

          return (
          <button
            key={item.label}
            type="button"
            onClick={() => {
              if (isMenu) navigate('/menu');
              else if (item.label === 'Dashboard') navigate('/dashboard');
              onClose?.();
            }}
            className={`flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-left text-sm transition ${
              isActive
                ? 'bg-[var(--color-primary)] text-black shadow-lg shadow-black/20'
                : 'text-[var(--color-muted)] hover:bg-[#171717] hover:text-[var(--color-text)]'
            }`}
          >
            <span className={`shrink-0 ${isActive ? 'opacity-100' : 'opacity-80'}`}>{item.icon}</span>
            <span className="truncate">{item.label}</span>
          </button>
          );
        })}
      </nav>
      </aside>
    </>
  );
};
