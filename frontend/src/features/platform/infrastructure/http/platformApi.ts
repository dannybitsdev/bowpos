import apiClient from '../../../auth/infrastructure/http/apiClient';
import type { PlatformTenant } from '../../domain/platformTypes';

export async function listTenants(): Promise<PlatformTenant[]> {
  const response = await apiClient.get<PlatformTenant[]>('/platform/tenants');
  return response.data;
}
