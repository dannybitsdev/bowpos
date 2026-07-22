import { create } from 'zustand';

import type { AuthUser } from '../domain/authTypes';

type AuthState = {
  accessToken: string | null;
  refreshToken: string | null;
  user: AuthUser | null;
  isAuthenticated: boolean;
  login: (tokens: { accessToken: string; refreshToken: string }, user: AuthUser) => void;
  rotateAccessToken: (tokens: { accessToken: string; refreshToken: string }) => void;
  logout: () => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  accessToken: null,
  refreshToken: null,
  user: null,
  isAuthenticated: false,
  login: (tokens, user) => {
    set({
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
      user,
      isAuthenticated: true,
    });
  },
  rotateAccessToken: (tokens) => {
    set((state) => ({
      ...state,
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
    }));
  },
  logout: () => {
    set({ accessToken: null, refreshToken: null, user: null, isAuthenticated: false });
  },
}));
