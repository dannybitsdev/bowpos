import apiClient from '../../../auth/infrastructure/http/apiClient';
import type { Branch } from '../../domain/branchTypes';

export async function listBranches(): Promise<Branch[]> {
  const response = await apiClient.get<Branch[]>('/locations');
  return response.data;
}
