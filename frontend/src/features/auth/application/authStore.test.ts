import { beforeEach, describe, expect, it } from 'vitest';

import { useAuthStore } from './authStore';

describe('authStore', () => {
  beforeEach(() => {
    useAuthStore.getState().logout();
  });

  it('sets authenticated state on login', () => {
    useAuthStore.getState().login({ accessToken: 'token-1', refreshToken: 'refresh-1' }, {
      user_id: 'user-1',
      tenant_id: 'tenant-1',
      role: 'SUPER_ADMIN',
      permissions: ['manage:tenant_admins'],
    });

    const state = useAuthStore.getState();
    expect(state.isAuthenticated).toBe(true);
    expect(state.accessToken).toBe('token-1');
    expect(state.refreshToken).toBe('refresh-1');
    expect(state.user?.role).toBe('SUPER_ADMIN');
  });

  it('clears state on logout', () => {
    useAuthStore.getState().login({ accessToken: 'token-1', refreshToken: 'refresh-1' }, {
      user_id: 'user-1',
      tenant_id: 'tenant-1',
      role: 'ADMIN_TENANT',
      permissions: ['manage:tenant_users'],
    });

    useAuthStore.getState().logout();
    const state = useAuthStore.getState();

    expect(state.isAuthenticated).toBe(false);
    expect(state.accessToken).toBeNull();
    expect(state.refreshToken).toBeNull();
    expect(state.user).toBeNull();
  });
});
