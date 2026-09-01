import axios from 'axios';

import { useAuthStore } from '../../application/authStore';
import { useBranchStore } from '../../../branch/application/branchStore';
import { usePlatformStore } from '../../../platform/application/platformStore';

const runtimeConfig = (globalThis as typeof globalThis & {
  __BOWPOS_CONFIG__?: { apiUrl?: string };
}).__BOWPOS_CONFIG__;
const configuredApiUrl = runtimeConfig?.apiUrl ?? import.meta.env.VITE_API_URL ?? '/api';
const apiBaseUrl = configuredApiUrl.replace(/\/$/, '').endsWith('/api')
  ? configuredApiUrl.replace(/\/$/, '')
  : `${configuredApiUrl.replace(/\/$/, '')}/api`;

const apiClient = axios.create({
  baseURL: apiBaseUrl,
  timeout: 12000,
});

const refreshClient = axios.create({
  baseURL: apiBaseUrl,
  timeout: 12000,
});

apiClient.interceptors.request.use((config) => {
  const { accessToken, user } = useAuthStore.getState();

  if (accessToken) {
    config.headers.Authorization = `Bearer ${accessToken}`;
  }

  if (user?.tenant_id) {
    config.headers['X-Tenant-ID'] = user.tenant_id;
  }

  const activeBranchId = useBranchStore.getState().activeBranchId;
  if (activeBranchId) {
    config.headers['X-Branch-ID'] = activeBranchId;
  }

  if (user?.role === 'SUPER_ADMIN') {
    const overrideTenantId = usePlatformStore.getState().overrideTenantId;
    if (overrideTenantId) {
      config.headers['X-Tenant-Override'] = overrideTenantId;
    }
  }

  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config as { _retry?: boolean; url?: string } | undefined;
    // Un 401 en login/refresh/logout debe resolverlo quien hizo la llamada (p.ej. mostrar
    // "credenciales inválidas" en el formulario), no el interceptor global: forzar aquí un
    // logout() + window.location.assign('/login') pisaba ese manejo y recargaba la SPA a
    // mitad del intento de inicio de sesión, aparentando un "logout inmediato tras login".
    const isAuthEndpoint = originalRequest?.url === '/v1/auth/logout'
      || originalRequest?.url === '/auth/login'
      || originalRequest?.url === '/auth/refresh';

    if (error.response?.status === 401 && originalRequest && !originalRequest._retry && !isAuthEndpoint) {
      const { refreshToken, rotateAccessToken, logout } = useAuthStore.getState();
      if (!refreshToken) {
        logout();
        window.location.assign('/login');
        return Promise.reject(error);
      }

      originalRequest._retry = true;

      try {
        const refreshResponse = await refreshClient.post('/auth/refresh', {
          refresh_token: refreshToken,
        });

        rotateAccessToken({
          accessToken: refreshResponse.data.tokens.access_token,
          refreshToken: refreshResponse.data.tokens.refresh_token,
        });

        originalRequest.headers = originalRequest.headers ?? {};
        originalRequest.headers.Authorization = `Bearer ${refreshResponse.data.tokens.access_token}`;
        return apiClient(originalRequest);
      } catch {
        logout();
        window.location.assign('/login');
        return Promise.reject(error);
      }
    }

    if (error.response?.status === 401 && !isAuthEndpoint) {
      useAuthStore.getState().logout();
      window.location.assign('/login');
    }

    return Promise.reject(error);
  }
);

export default apiClient;
