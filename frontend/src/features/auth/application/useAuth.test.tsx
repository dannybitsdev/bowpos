import { act, render, renderHook, screen } from '@testing-library/react';
import { MemoryRouter, Route, Routes, useLocation } from 'react-router-dom';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { useAuthStore } from './authStore';
import { useBranchStore } from '../../branch/application/branchStore';
import { usePlatformStore } from '../../platform/application/platformStore';
import { useAuth } from './useAuth';

vi.mock('../infrastructure/http/authApi', () => ({
  logoutRequest: vi.fn().mockResolvedValue(undefined),
}));

function LocationProbe() {
  const location = useLocation();
  return <span data-testid="pathname">{location.pathname}</span>;
}

function LogoutButton() {
  const { logout } = useAuth();
  return (
    <button type="button" onClick={() => void logout()}>
      Cerrar sesión
    </button>
  );
}

function renderWithRouter() {
  return render(
    <MemoryRouter initialEntries={['/dashboard']}>
      <Routes>
        <Route
          path="/dashboard"
          element={
            <>
              <LocationProbe />
              <LogoutButton />
            </>
          }
        />
        <Route path="/login" element={<LocationProbe />} />
      </Routes>
    </MemoryRouter>
  );
}

describe('useAuth().logout', () => {
  afterEach(() => {
    useAuthStore.getState().logout();
    useBranchStore.getState().setActiveBranchId(null);
    usePlatformStore.getState().setOverrideTenantId(null);
    vi.clearAllMocks();
  });

  it('clears the global auth state and redirects to /login', async () => {
    useAuthStore.getState().login({ accessToken: 'token', refreshToken: 'refresh' }, {
      user_id: 'user-1', tenant_id: 'tenant-1', name: 'Andres Gomez', role: 'ADMIN_TENANT', permissions: ['dashboard:read'],
    });
    useBranchStore.getState().setActiveBranchId('branch-1');
    usePlatformStore.getState().setOverrideTenantId('tenant-override');

    renderWithRouter();

    await act(async () => {
      screen.getByRole('button', { name: 'Cerrar sesión' }).click();
    });

    const authState = useAuthStore.getState();
    expect(authState.isAuthenticated).toBe(false);
    expect(authState.accessToken).toBeNull();
    expect(authState.refreshToken).toBeNull();
    expect(authState.user).toBeNull();
    expect(useBranchStore.getState().activeBranchId).toBeNull();
    expect(usePlatformStore.getState().overrideTenantId).toBeNull();
    expect(screen.getByTestId('pathname').textContent).toBe('/login');
  });

  it('still clears local session and redirects even if the backend call fails', async () => {
    const { logoutRequest } = await import('../infrastructure/http/authApi');
    vi.mocked(logoutRequest).mockRejectedValueOnce(new Error('network error'));

    useAuthStore.getState().login({ accessToken: 'token', refreshToken: 'refresh' }, {
      user_id: 'user-1', tenant_id: 'tenant-1', name: 'Andres Gomez', role: 'CAJERO', permissions: ['ventas:create'],
    });

    const { result } = renderHook(() => useAuth(), {
      wrapper: ({ children }) => <MemoryRouter>{children}</MemoryRouter>,
    });

    await act(async () => {
      await result.current.logout();
    });

    expect(useAuthStore.getState().isAuthenticated).toBe(false);
  });
});
