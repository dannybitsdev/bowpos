import { create } from 'zustand';

import type { AuthUser } from '../domain/authTypes';

const AUTH_STORAGE_KEY = 'bowpos.auth.session';

type AuthState = {
  accessToken: string | null;
  refreshToken: string | null;
  user: AuthUser | null;
  isAuthenticated: boolean;
  login: (tokens: { accessToken: string; refreshToken: string }, user: AuthUser) => void;
  rotateAccessToken: (tokens: { accessToken: string; refreshToken: string }) => void;
  logout: () => void;
};

type PersistedAuthState = Pick<AuthState, 'accessToken' | 'refreshToken' | 'user'>;

function readPersistedSession(): PersistedAuthState {
  const storage = getStorage();
  if (!storage) return { accessToken: null, refreshToken: null, user: null };
  try {
    const stored = storage.getItem(AUTH_STORAGE_KEY);
    if (!stored) return { accessToken: null, refreshToken: null, user: null };
    const session = JSON.parse(stored) as PersistedAuthState;
    return { accessToken: session.accessToken ?? null, refreshToken: session.refreshToken ?? null, user: session.user ?? null };
  } catch {
    storage.removeItem(AUTH_STORAGE_KEY);
    return { accessToken: null, refreshToken: null, user: null };
  }
}

function getStorage(): Storage | null {
  try {
    return typeof window !== 'undefined' && window.localStorage ? window.localStorage : null;
  } catch {
    return null;
  }
}

function persistSession(state: PersistedAuthState) {
  getStorage()?.setItem(AUTH_STORAGE_KEY, JSON.stringify(state));
}

function clearPersistedSession() {
  getStorage()?.removeItem(AUTH_STORAGE_KEY);
}

const persistedSession = readPersistedSession();

export const useAuthStore = create<AuthState>((set) => ({
  ...persistedSession,
  isAuthenticated: Boolean(persistedSession.accessToken && persistedSession.refreshToken && persistedSession.user),
  login: (tokens, user) => {
    const session = {
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
      user,
    };
    persistSession(session);
    set({ ...session, isAuthenticated: true });
  },
  rotateAccessToken: (tokens) => {
    set((state) => {
      const session = {
        accessToken: tokens.accessToken,
        refreshToken: tokens.refreshToken,
        user: state.user,
      };
      persistSession(session);
      return {
      ...state,
      accessToken: tokens.accessToken,
      refreshToken: tokens.refreshToken,
      };
    });
  },
  logout: () => {
    clearPersistedSession();
    set({ accessToken: null, refreshToken: null, user: null, isAuthenticated: false });
  },
}));
