import { useNavigate } from 'react-router-dom';

import { useAuthStore } from './authStore';
import { useBranchStore } from '../../branch/application/branchStore';
import { usePlatformStore } from '../../platform/application/platformStore';
import { logoutRequest } from '../infrastructure/http/authApi';

/**
 * Cierre de sesión atómico: revoca el token en el servidor, limpia todo el
 * estado global (auth, sede activa, override de tenant) y su persistencia,
 * y redirige a /login incluso si la petición al backend falla.
 */
export function useAuth() {
  const navigate = useNavigate();

  async function logout() {
    const { refreshToken } = useAuthStore.getState();

    try {
      await logoutRequest(refreshToken);
    } catch {
      // La sesión local se limpia igual aunque el backend no responda.
    } finally {
      useAuthStore.getState().logout();
      useBranchStore.getState().setActiveBranchId(null);
      usePlatformStore.getState().setOverrideTenantId(null);
      navigate('/login', { replace: true });
    }
  }

  return { logout };
}
