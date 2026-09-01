import { create } from 'zustand';

import type { PlatformTenant } from '../domain/platformTypes';
import { useBranchStore } from '../../branch/application/branchStore';

const TENANT_OVERRIDE_STORAGE_KEY = 'bowpos.platform.tenantOverride';

type PlatformState = {
  tenants: PlatformTenant[];
  overrideTenantId: string | null;
  setTenants: (tenants: PlatformTenant[]) => void;
  setOverrideTenantId: (tenantId: string | null) => void;
};

function getStorage(): Storage | null {
  try {
    return typeof window !== 'undefined' && window.localStorage ? window.localStorage : null;
  } catch {
    return null;
  }
}

export const usePlatformStore = create<PlatformState>((set) => ({
  tenants: [],
  overrideTenantId: getStorage()?.getItem(TENANT_OVERRIDE_STORAGE_KEY) ?? null,
  setTenants: (tenants) => set({ tenants }),
  setOverrideTenantId: (tenantId) => {
    const storage = getStorage();
    if (tenantId) storage?.setItem(TENANT_OVERRIDE_STORAGE_KEY, tenantId);
    else storage?.removeItem(TENANT_OVERRIDE_STORAGE_KEY);
    set({ overrideTenantId: tenantId });
    // Cambiar de tenant invalida la sede activa: las sedes pertenecen a un tenant concreto.
    useBranchStore.getState().setActiveBranchId(null);
    useBranchStore.getState().setBranches([]);
  },
}));
