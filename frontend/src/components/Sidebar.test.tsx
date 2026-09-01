import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { useAuthStore } from '../features/auth/application/authStore';
import { Sidebar } from './Sidebar';

vi.mock('../features/branch/application/BranchContext', () => ({
  useBranchContext: () => ({ branches: [], activeBranchId: null, setActiveBranchId: vi.fn() }),
}));
vi.mock('../features/platform/application/PlatformContext', () => ({
  usePlatformContext: () => ({ tenants: [], overrideTenantId: null, setOverrideTenantId: vi.fn() }),
}));

describe('Sidebar', () => {
  afterEach(() => useAuthStore.getState().logout());

  it('does not render sections without a granted permission', () => {
    useAuthStore.getState().login({ accessToken: 'token', refreshToken: 'refresh' }, {
      user_id: 'waiter', tenant_id: 'tenant', role: 'MESERO', permissions: ['ordenes:create'],
    });

    render(<MemoryRouter><Sidebar /></MemoryRouter>);

    expect(screen.queryByRole('button', { name: 'Inventarios' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Configuración' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Reportes' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Cumplimiento Legal' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Ventas' })).not.toBeInTheDocument();
  });
});