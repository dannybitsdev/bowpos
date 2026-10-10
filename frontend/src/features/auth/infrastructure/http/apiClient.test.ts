import axios from 'axios';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { useAuthStore } from '../../application/authStore';

describe('apiClient refresh handling', () => {
  afterEach(() => {
    useAuthStore.getState().logout();
    vi.restoreAllMocks();
  });

  it('keeps a cashier session across branch 403s and concurrent 401 refreshes', async () => {
    const clients: ReturnType<typeof axios.create>[] = [];
    const create = axios.create.bind(axios);
    vi.spyOn(axios, 'create').mockImplementation((config) => {
      const client = create(config);
      clients.push(client);
      return client;
    });

    const { default: apiClient } = await import('./apiClient');
    const [requestClient, refreshClient] = clients;
    useAuthStore.getState().login({ accessToken: 'expired', refreshToken: 'refresh-old' }, {
      user_id: 'user-1',
      tenant_id: 'tenant-1',
      role: 'CAJERO',
      permissions: ['dashboard:read', 'ventas:read', 'ventas:create', 'ordenes:read', 'ordenes:create'],
    });

    let resolveRefresh!: (response: unknown) => void;
    vi.spyOn(refreshClient, 'post').mockReturnValue(new Promise((resolve) => {
      resolveRefresh = resolve;
    }) as never);
    const adapter = vi.fn(async (config) => ({
      data: {},
      status: 200,
      statusText: 'OK',
      headers: {},
      config,
    }));
    requestClient.defaults.adapter = adapter;

    const responseInterceptors = apiClient.interceptors.response as unknown as {
      handlers: Array<{ rejected: (error: unknown) => Promise<unknown> }>;
    };
    const rejectResponse = responseInterceptors.handlers[0].rejected;

    const rejectUnauthorized = () => rejectResponse({
      config: { url: '/locations', headers: {} },
      response: { status: 401 },
    });
    const requests = [rejectUnauthorized(), rejectUnauthorized()];

    expect(refreshClient.post).toHaveBeenCalledTimes(1);
    resolveRefresh({
      data: { tokens: { access_token: 'access-new', refresh_token: 'refresh-new' } },
    });

    await Promise.all(requests);

    expect(adapter).toHaveBeenCalledTimes(2);
    expect(useAuthStore.getState().accessToken).toBe('access-new');
    expect(useAuthStore.getState().refreshToken).toBe('refresh-new');
    expect(useAuthStore.getState().isAuthenticated).toBe(true);

    const branchForbidden = {
      config: { url: '/locations', headers: {} },
      response: { status: 403 },
    };
    await expect(rejectResponse(branchForbidden)).rejects.toBe(branchForbidden);
    expect(refreshClient.post).toHaveBeenCalledTimes(1);
    expect(useAuthStore.getState().isAuthenticated).toBe(true);
  });
});