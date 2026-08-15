import axios from 'axios';

import { useAuthStore } from '../../application/authStore';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL ?? '/api',
  timeout: 12000,
});

const refreshClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL ?? '/api',
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

  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config as { _retry?: boolean } | undefined;

    if (error.response?.status === 401 && originalRequest && !originalRequest._retry) {
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

    if (error.response?.status === 401) {
      useAuthStore.getState().logout();
      window.location.assign('/login');
    }

    return Promise.reject(error);
  }
);

export default apiClient;
