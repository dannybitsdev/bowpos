import { useState } from 'react';
import { Navigate, Outlet, Route, Routes } from 'react-router-dom';

import { Sidebar } from '../components/Sidebar';
import { Dashboard } from '../pages/Dashboard';
import { MenuPage } from '../pages/MenuPage';
import { CategoryManagementPage } from '../pages/CategoryManagementPage';
import { OrderBuilderPage } from '../pages/OrderBuilderPage';
import { OrdersPage } from '../pages/OrdersPage';
import { LoginPage } from '../features/auth/presentation/pages/LoginPage';
import { ProtectedRoute } from '../features/auth/presentation/components/ProtectedRoute';
import { BranchProvider } from '../features/branch/application/BranchContext';
import { PlatformProvider } from '../features/platform/application/PlatformContext';

function UnauthorizedPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-950 px-4 text-zinc-100">
      <div className="w-full max-w-md rounded-2xl border border-zinc-800 bg-zinc-900 p-6 text-center sm:p-8">
        <h1 className="text-2xl font-semibold">Acceso denegado</h1>
        <p className="mt-2 text-sm text-zinc-400">No tienes permisos para acceder a esta sección.</p>
      </div>
    </div>
  );
}

function DashboardLayout() {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <BranchProvider>
      <PlatformProvider>
        <div className="flex min-h-screen overflow-x-hidden bg-[var(--color-background)]">
        <Sidebar open={sidebarOpen} onClose={() => setSidebarOpen(false)} />
        <main className="min-w-0 flex-1 lg:ml-72">
          <header className="screen-only sticky top-0 z-20 flex items-center gap-3 border-b border-[var(--color-border)] bg-[var(--color-background)]/95 px-4 py-3 backdrop-blur lg:hidden">
            <button
              type="button"
              onClick={() => setSidebarOpen(true)}
              className="flex h-10 w-10 items-center justify-center rounded-xl border border-[var(--color-border)] text-[var(--color-text)] transition hover:border-[var(--color-primary)]"
              aria-label="Abrir navegación"
              aria-expanded={sidebarOpen}
            >
              <span className="sr-only">Abrir navegación</span>
              <svg viewBox="0 0 24 24" className="h-5 w-5" fill="none" stroke="currentColor" strokeWidth="1.8" aria-hidden="true">
                <path d="M4 7h16M4 12h16M4 17h16" />
              </svg>
            </button>
            <span className="truncate text-sm font-semibold text-[var(--color-text)]">Panel central</span>
          </header>
          <div className="min-w-0"><Outlet /></div>
        </main>
      </div>
      </PlatformProvider>
    </BranchProvider>
  );
}

export function AppRouter() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/unauthorized" element={<UnauthorizedPage />} />
      <Route
        element={
          <ProtectedRoute allowedRoles={['SUPER_ADMIN', 'ADMIN_TENANT', 'BRANCH_MANAGER', 'CAJERO', 'MESERO']}>
            <DashboardLayout />
          </ProtectedRoute>
        }
      >
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/menu" element={<ProtectedRoute allowedRoles={['SUPER_ADMIN', 'ADMIN_TENANT', 'BRANCH_MANAGER']}><MenuPage /></ProtectedRoute>} />
        <Route path="/categories" element={<ProtectedRoute allowedRoles={['SUPER_ADMIN', 'ADMIN_TENANT', 'BRANCH_MANAGER']}><CategoryManagementPage /></ProtectedRoute>} />
        <Route path="/orders" element={<OrderBuilderPage />} />
        <Route path="/orders/history" element={<OrdersPage />} />
      </Route>
      <Route path="*" element={<Navigate to="/dashboard" replace />} />
    </Routes>
  );
}
