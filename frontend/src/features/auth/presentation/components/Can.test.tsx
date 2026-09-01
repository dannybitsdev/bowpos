import { renderHook } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';

import { useAuthStore } from '../../application/authStore';
import { usePermission } from './Can';

describe('usePermission', () => {
  afterEach(() => useAuthStore.getState().logout());

  it('only grants permissions included in the authenticated session', () => {
    useAuthStore.getState().login({ accessToken: 'token', refreshToken: 'refresh' }, {
      user_id: 'cashier', tenant_id: 'tenant', role: 'CAJERO', permissions: ['ventas:create'],
    });

    const { result } = renderHook(() => usePermission('ventas:cancel'));

    expect(result.current).toBe(false);
  });
});