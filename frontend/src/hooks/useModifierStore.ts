import { create } from 'zustand';

import type { ModifierGroup } from '../pages/menuTypes';

type ModifierState = {
  modifierGroups: ModifierGroup[];
  loading: boolean;
  error: string | null;
  setModifierGroups: (groups: ModifierGroup[]) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  upsertGroup: (group: ModifierGroup) => void;
  removeGroup: (groupId: string) => void;
};

/** Client-side cache of modifier groups/modifiers; mutated by `useProductModifiers`. */
export const useModifierStore = create<ModifierState>((set) => ({
  modifierGroups: [],
  loading: false,
  error: null,
  setModifierGroups: (modifierGroups) => set({ modifierGroups }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  upsertGroup: (group) =>
    set((state) => {
      const exists = state.modifierGroups.some((item) => item.id === group.id);
      return {
        modifierGroups: exists
          ? state.modifierGroups.map((item) => (item.id === group.id ? group : item))
          : [...state.modifierGroups, group],
      };
    }),
  removeGroup: (groupId) =>
    set((state) => ({ modifierGroups: state.modifierGroups.filter((item) => item.id !== groupId) })),
}));
