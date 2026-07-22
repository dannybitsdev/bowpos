import { Navigate, Route, Routes } from 'react-router-dom';

import { Sidebar } from '../components/Sidebar';
import { Dashboard } from '../pages/Dashboard';
import { LoginPage } from '../features/auth/presentation/pages/LoginPage';
import { ProtectedRoute } from '../features/auth/presentation/components/ProtectedRoute';

function UnauthorizedPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-950 px-4 text-zinc-100">
      <div className="rounded-2xl border border-zinc-800 bg-zinc-900 p-8 text-center">
        <h1 className="text-2xl font-semibold">Acceso denegado</h1>
        <p className="mt-2 text-sm text-zinc-400">No tienes permisos para acceder a esta sección.</p>
      </div>
    </div>
  );
}

function DashboardLayout() {
  return (
    <div className="flex min-h-screen bg-[var(--color-background)]">
      <Sidebar />
      <main className="flex-1">
        <Dashboard />
      </main>
    </div>
  );
}

export function AppRouter() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/unauthorized" element={<UnauthorizedPage />} />
      <Route
        path="/dashboard"
        element={
          <ProtectedRoute allowedRoles={['SUPER_ADMIN', 'ADMIN_TENANT', 'CAJERO', 'MESERO']}>
            <DashboardLayout />
          </ProtectedRoute>
        }
      />
      <Route path="*" element={<Navigate to="/dashboard" replace />} />
    </Routes>
  );
}
