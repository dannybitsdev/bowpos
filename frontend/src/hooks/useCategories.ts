import { useCallback, useEffect, useState } from 'react';

import apiClient from '../features/auth/infrastructure/http/apiClient';
import type { CategoryPayload, MenuCategory } from '../pages/menuTypes';

export function useCategories() {
  const [categories, setCategories] = useState<MenuCategory[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadCategories = useCallback(async () => {
    setLoading(true);
    try {
      const response = await apiClient.get<{ data: MenuCategory[] }>('/v1/menu/categories');
      setCategories(response.data.data);
      setError(null);
    } catch {
      setError('No fue posible cargar las categorías.');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadCategories();
  }, [loadCategories]);

  const createCategory = useCallback(async (payload: CategoryPayload) => {
    await apiClient.post('/v1/menu/categories', payload);
    await loadCategories();
  }, [loadCategories]);

  const updateCategory = useCallback(async (categoryId: string, payload: CategoryPayload) => {
    await apiClient.put(`/v1/menu/categories/${categoryId}`, payload);
    await loadCategories();
  }, [loadCategories]);

  const deactivateCategory = useCallback(async (categoryId: string) => {
    await apiClient.delete(`/v1/menu/categories/${categoryId}`);
    await loadCategories();
  }, [loadCategories]);

  return { categories, loading, error, loadCategories, createCategory, updateCategory, deactivateCategory };
}
