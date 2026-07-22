import { Navigate } from 'react-router-dom';

import type { Role } from '../../domain/authTypes';
import { useAuthStore } from '../../application/authStore';

type ProtectedRouteProps = {
  allowedRoles: Role[];
  children: JSX.Element;
};

export function ProtectedRoute({ allowedRoles, children }: ProtectedRouteProps) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const role = useAuthStore((state) => state.user?.role);

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  if (!role || !allowedRoles.includes(role)) {
    return <Navigate to="/unauthorized" replace />;
  }

  return children;
}
