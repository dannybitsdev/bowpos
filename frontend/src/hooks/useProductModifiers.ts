import { useCallback, useEffect } from 'react';

import apiClient from '../features/auth/infrastructure/http/apiClient';
import type { ModifierGroup, ModifierGroupPayload, ModifierPayload } from '../pages/menuTypes';
import { useModifierStore } from './useModifierStore';

/** Business logic for the Modifier/Add-on CRUD; keeps `ModifierGroupsPage` a thin presentational shell. */
export function useProductModifiers() {
  const modifierGroups = useModifierStore((state) => state.modifierGroups);
  const loading = useModifierStore((state) => state.loading);
  const error = useModifierStore((state) => state.error);
  const setModifierGroups = useModifierStore((state) => state.setModifierGroups);
  const setLoading = useModifierStore((state) => state.setLoading);
  const setError = useModifierStore((state) => state.setError);

  const loadModifierGroups = useCallback(async () => {
    setLoading(true);
    try {
      const response = await apiClient.get<{ data: ModifierGroup[] }>('/v1/menu/modifier-groups');
      setModifierGroups(response.data.data);
      setError(null);
    } catch {
      setError('No fue posible cargar los grupos de modificadores.');
    } finally {
      setLoading(false);
    }
  }, [setModifierGroups, setLoading, setError]);

  useEffect(() => {
    void loadModifierGroups();
  }, [loadModifierGroups]);

  const createModifierGroup = useCallback(async (payload: ModifierGroupPayload) => {
    await apiClient.post('/v1/menu/modifier-groups', payload);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const updateModifierGroup = useCallback(async (groupId: string, payload: ModifierGroupPayload) => {
    await apiClient.put(`/v1/menu/modifier-groups/${groupId}`, payload);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const deactivateModifierGroup = useCallback(async (groupId: string) => {
    await apiClient.delete(`/v1/menu/modifier-groups/${groupId}`);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const createModifier = useCallback(async (groupId: string, payload: ModifierPayload) => {
    await apiClient.post(`/v1/menu/modifier-groups/${groupId}/modifiers`, payload);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const updateModifier = useCallback(async (modifierId: string, payload: ModifierPayload) => {
    await apiClient.put(`/v1/menu/modifiers/${modifierId}`, payload);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const deleteModifier = useCallback(async (modifierId: string) => {
    await apiClient.delete(`/v1/menu/modifiers/${modifierId}`);
    await loadModifierGroups();
  }, [loadModifierGroups]);

  const getProductModifierGroupIds = useCallback(async (productId: string) => {
    const response = await apiClient.get<{ data: string[] }>(`/v1/menu/products/${productId}/modifier-groups`);
    return response.data.data;
  }, []);

  const setProductModifierGroups = useCallback(async (productId: string, modifierGroupIds: string[]) => {
    await apiClient.put(`/v1/menu/products/${productId}/modifier-groups`, { modifier_group_ids: modifierGroupIds });
  }, []);

  return {
    modifierGroups,
    loading,
    error,
    loadModifierGroups,
    createModifierGroup,
    updateModifierGroup,
    deactivateModifierGroup,
    createModifier,
    updateModifier,
    deleteModifier,
    getProductModifierGroupIds,
    setProductModifierGroups,
  };
}
