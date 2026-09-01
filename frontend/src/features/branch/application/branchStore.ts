import { create } from 'zustand';

import type { Branch } from '../domain/branchTypes';

const BRANCH_STORAGE_KEY = 'bowpos.branch.active';

type BranchState = {
  branches: Branch[];
  activeBranchId: string | null;
  setBranches: (branches: Branch[]) => void;
  setActiveBranchId: (branchId: string | null) => void;
};

function getStorage(): Storage | null {
  try {
    return typeof window !== 'undefined' && window.localStorage ? window.localStorage : null;
  } catch {
    return null;
  }
}

function readPersistedBranchId(): string | null {
  return getStorage()?.getItem(BRANCH_STORAGE_KEY) ?? null;
}

export const useBranchStore = create<BranchState>((set, get) => ({
  branches: [],
  activeBranchId: readPersistedBranchId(),
  setBranches: (branches) => {
    set({ branches });
    const current = get().activeBranchId;
    const stillValid = current && branches.some((branch) => branch.id === current);
    if (!stillValid) {
      // Con una sola sede accesible, se selecciona autom\u00e1ticamente; con m\u00faltiples, el usuario elige.
      const next = branches.length === 1 ? branches[0].id : null;
      get().setActiveBranchId(next);
    }
  },
  setActiveBranchId: (branchId) => {
    const storage = getStorage();
    if (branchId) storage?.setItem(BRANCH_STORAGE_KEY, branchId);
    else storage?.removeItem(BRANCH_STORAGE_KEY);
    set({ activeBranchId: branchId });
  },
}));
