import React from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import { useAuthStore } from '../features/auth/application/authStore';
import { useBranchContext } from '../features/branch/application/BranchContext';
import { usePlatformContext } from '../features/platform/application/PlatformContext';
import type { Permission } from '../features/auth/domain/authTypes';
import { useAuth } from '../features/auth/application/useAuth';
import { useConfirmationModal } from '../features/modal/presentation/useConfirmationModal';

type NavItem = {
  label: string;
  path: string;
  requiredPermissions: Permission[];
  icon: React.ReactNode;
};

const navItems: NavItem[] = [
  { label: 'Dashboard', path: '/dashboard', requiredPermissions: ['dashboard:read'], icon: <DashboardIcon /> },
  { label: 'Ventas', path: '/orders', requiredPermissions: ['ventas:create'], icon: <SalesIcon /> },
  { label: 'Órdenes', path: '/orders/history', requiredPermissions: ['ordenes:read'], icon: <OrdersIcon /> },
  { label: 'Menú', path: '/menu', requiredPermissions: ['inventario:read'], icon: <MenuIcon /> },
  { label: 'Categorías', path: '/categories', requiredPermissions: ['inventario:admin'], icon: <CategoriesIcon /> },
  { label: 'Modificadores', path: '/modifiers', requiredPermissions: ['inventario:admin'], icon: <ModifiersIcon /> },
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

function CategoriesIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden="true">
      <rect x="4" y="4" width="6" height="6" rx="1" />
      <rect x="14" y="4" width="6" height="6" rx="1" />
      <rect x="4" y="14" width="6" height="6" rx="1" />
      <rect x="14" y="14" width="6" height="6" rx="1" />
    </svg>
  );
}

function ModifiersIcon() {
  return (
    <svg viewBox="0 0 24 24" className="h-4 w-4" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden="true">
      <circle cx="12" cy="12" r="3" />
      <path d="M12 3v3M12 18v3M3 12h3M18 12h3M5.6 5.6l2.1 2.1M16.3 16.3l2.1 2.1M18.4 5.6l-2.1 2.1M7.7 16.3l-2.1 2.1" />
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
  const user = useAuthStore((state) => state.user);
  const role = user?.role;
  const permissions = user?.permissions ?? [];
  const tenantName = user?.tenant_name ?? 'Bits TI Tecnología';
  const { branches, activeBranchId, setActiveBranchId } = useBranchContext();
  const { tenants, overrideTenantId, setOverrideTenantId } = usePlatformContext();
  const { logout } = useAuth();
  const { confirm } = useConfirmationModal();
  const activeBranchName = branches.find((branch) => branch.id === activeBranchId)?.name ?? 'Todas las sedes';
  const roleLabel = roleLabels[role ?? 'CAJERO'];
  const initials = (user?.name ?? tenantName)
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0])
    .join('')
    .toUpperCase();

  const handleLogout = async () => {
    const confirmed = await confirm({
      title: 'Cerrar sesión',
      description: 'Deberás iniciar sesión nuevamente para continuar operando en el sistema.',
      confirmLabel: 'Cerrar sesión',
      cancelLabel: 'Cancelar',
      variant: 'warning',
    });
    if (confirmed) {
      void logout();
    }
  };

  return (
    <>
      {open ? <button type="button" aria-label="Cerrar navegación" onClick={onClose} className="fixed inset-0 z-30 bg-black/70 lg:hidden" /> : null}
      <aside className={`fixed inset-y-0 left-0 z-40 flex w-[min(18rem,calc(100vw-2rem))] flex-col border-r border-[var(--color-border)] bg-[var(--color-sidebar-bg)] px-5 py-6 text-[var(--color-text)] shadow-2xl transition-transform duration-200 lg:z-30 lg:h-screen lg:translate-x-0 lg:shadow-none ${open ? 'translate-x-0' : '-translate-x-full'}`}>
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
        <select
          value={activeBranchId ?? ''}
          onChange={(event) => setActiveBranchId(event.target.value || null)}
          className="w-full rounded-xl border border-[var(--color-border)] bg-[#101010] px-3 py-2.5 text-sm text-[var(--color-text)] outline-none transition focus:border-[var(--color-primary)]"
        >
          {branches.length > 1 ? <option value="">Todas las sedes</option> : null}
          {branches.map((branch) => <option key={branch.id} value={branch.id}>{branch.name}</option>)}
          {branches.length === 0 ? <option value="">Sin sedes asignadas</option> : null}
        </select>
      </label>

      {role === 'SUPER_ADMIN' ? (
        <label className="mb-6 block text-sm text-[var(--color-muted)]">
          <span className="mb-2 block text-[11px] uppercase tracking-[0.3em] text-[var(--color-primary)]">Modo plataforma · Tenant</span>
          <select
            value={overrideTenantId ?? ''}
            onChange={(event) => setOverrideTenantId(event.target.value || null)}
            className="w-full rounded-xl border border-[var(--color-border)] bg-[#101010] px-3 py-2.5 text-sm text-[var(--color-text)] outline-none transition focus:border-[var(--color-primary)]"
          >
            <option value="">Mi tenant</option>
            {tenants.map((tenant) => <option key={tenant.id} value={tenant.id}>{tenant.name}</option>)}
          </select>
        </label>
      ) : null}

      <nav className="min-h-0 flex-1 space-y-1.5 overflow-y-auto pr-1">
        {navItems.filter((item) => item.requiredPermissions.every((permission) => permissions.includes(permission))).map((item) => {
          const isActive = item.path === '/orders/history'
            ? location.pathname === '/orders/history'
            : location.pathname === item.path;

          return (
          <button
            key={item.label}
            type="button"
            onClick={() => {
              navigate(item.path);
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
      <div className="mt-5 border-t border-[var(--color-border)] pt-4">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-[var(--color-primary)] text-sm font-bold text-black" aria-hidden="true">
            {initials}
          </div>
          <div className="min-w-0 flex-1">
            <p className="truncate text-sm font-semibold text-[var(--color-text)]">{user?.name ?? 'Usuario'}</p>
            <p className="truncate text-xs text-[var(--color-muted)]">{roleLabel} · {activeBranchName}</p>
          </div>
          <button type="button" onClick={() => void handleLogout()} className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-[var(--color-border)] text-[var(--color-muted)] transition hover:border-red-400 hover:text-red-300" aria-label="Cerrar sesión" title="Cerrar sesión">
            <span aria-hidden="true">↪</span>
          </button>
        </div>
      </div>
      </aside>
    </>
  );
};

const roleLabels = {
  SUPER_ADMIN: 'Super Admin',
  ADMIN_TENANT: 'Administrador',
  BRANCH_MANAGER: 'Gerente de sede',
  CAJERO: 'Cajero',
  MESERO: 'Mesero',
  COCINERO: 'Cocinero',
} as const;
