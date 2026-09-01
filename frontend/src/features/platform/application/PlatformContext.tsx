import { createContext, useContext, useEffect, type ReactNode } from 'react';

import { useAuthStore } from '../../auth/application/authStore';
import { listTenants } from '../infrastructure/http/platformApi';
import { usePlatformStore } from './platformStore';

type PlatformContextValue = {
  tenants: ReturnType<typeof usePlatformStore.getState>['tenants'];
  overrideTenantId: string | null;
  setOverrideTenantId: (tenantId: string | null) => void;
};

const PlatformContext = createContext<PlatformContextValue | null>(null);

export function PlatformProvider({ children }: { children: ReactNode }) {
  const role = useAuthStore((state) => state.user?.role);
  const tenants = usePlatformStore((state) => state.tenants);
  const overrideTenantId = usePlatformStore((state) => state.overrideTenantId);
  const setTenants = usePlatformStore((state) => state.setTenants);
  const setOverrideTenantId = usePlatformStore((state) => state.setOverrideTenantId);

  useEffect(() => {
    if (role !== 'SUPER_ADMIN') return;
    void listTenants().then(setTenants).catch(() => setTenants([]));
  }, [role, setTenants]);

  return <PlatformContext.Provider value={{ tenants, overrideTenantId, setOverrideTenantId }}>{children}</PlatformContext.Provider>;
}

export function usePlatformContext() {
  const context = useContext(PlatformContext);
  if (!context) {
    throw new Error('usePlatformContext must be used inside PlatformProvider');
  }
  return context;
}
