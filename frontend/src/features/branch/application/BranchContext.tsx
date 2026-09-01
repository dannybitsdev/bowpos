import { createContext, useContext, useEffect, type ReactNode } from 'react';

import { useAuthStore } from '../../auth/application/authStore';
import { usePlatformStore } from '../../platform/application/platformStore';
import { listBranches } from '../infrastructure/http/branchApi';
import { useBranchStore } from './branchStore';

type BranchContextValue = {
  branches: ReturnType<typeof useBranchStore.getState>['branches'];
  activeBranchId: string | null;
  setActiveBranchId: (branchId: string | null) => void;
};

const BranchContext = createContext<BranchContextValue | null>(null);

export function BranchProvider({ children }: { children: ReactNode }) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const overrideTenantId = usePlatformStore((state) => state.overrideTenantId);
  const branches = useBranchStore((state) => state.branches);
  const activeBranchId = useBranchStore((state) => state.activeBranchId);
  const setBranches = useBranchStore((state) => state.setBranches);
  const setActiveBranchId = useBranchStore((state) => state.setActiveBranchId);

  useEffect(() => {
    if (!isAuthenticated) return;
    // Reconsulta sedes cuando un SUPER_ADMIN cambia de tenant en modo plataforma.
    void listBranches().then(setBranches).catch(() => setBranches([]));
  }, [isAuthenticated, overrideTenantId, setBranches]);

  return <BranchContext.Provider value={{ branches, activeBranchId, setActiveBranchId }}>{children}</BranchContext.Provider>;
}

export function useBranchContext() {
  const context = useContext(BranchContext);
  if (!context) {
    throw new Error('useBranchContext must be used inside BranchProvider');
  }
  return context;
}
