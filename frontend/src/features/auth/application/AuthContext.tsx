import { createContext, useContext, type ReactNode } from 'react';

import { useAuthStore } from './authStore';

type AuthContextValue = {
  login: ReturnType<typeof useAuthStore.getState>['login'];
  rotateAccessToken: ReturnType<typeof useAuthStore.getState>['rotateAccessToken'];
  logout: ReturnType<typeof useAuthStore.getState>['logout'];
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const login = useAuthStore((state) => state.login);
  const rotateAccessToken = useAuthStore((state) => state.rotateAccessToken);
  const logout = useAuthStore((state) => state.logout);

  return <AuthContext.Provider value={{ login, rotateAccessToken, logout }}>{children}</AuthContext.Provider>;
}

export function useAuthContext() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuthContext must be used inside AuthProvider');
  }
  return context;
}
