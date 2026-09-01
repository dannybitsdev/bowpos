import apiClient from './apiClient';
import type { LoginResult } from '../../domain/authTypes';

export async function loginRequest(email: string, password: string, tenantId?: string): Promise<LoginResult> {
  const response = await apiClient.post<LoginResult>('/auth/login', {
    email,
    password,
    tenant_id: tenantId,
  });

  return response.data;
}

export async function refreshTokenRequest(refreshToken: string): Promise<LoginResult> {
  const response = await apiClient.post<LoginResult>('/auth/refresh', {
    refresh_token: refreshToken,
  });

  return response.data;
}

export async function logoutRequest(refreshToken: string | null): Promise<void> {
  await apiClient.post('/v1/auth/logout', { refresh_token: refreshToken });
}
