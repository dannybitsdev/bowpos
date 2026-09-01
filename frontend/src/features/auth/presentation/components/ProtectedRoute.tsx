import { Navigate } from 'react-router-dom';

import type { Permission, Role } from '../../domain/authTypes';
import { useAuthStore } from '../../application/authStore';

type ProtectedRouteProps = {
  allowedRoles?: Role[];
  requiredPermissions?: Permission[];
  children: JSX.Element;
};

export function ProtectedRoute({ allowedRoles, requiredPermissions = [], children }: ProtectedRouteProps) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const user = useAuthStore((state) => state.user);

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  const hasAllowedRole = !allowedRoles || (user != null && allowedRoles.includes(user.role));
  const hasPermissions = requiredPermissions.every((permission) => user?.permissions.includes(permission));
  if (!hasAllowedRole || !hasPermissions) {
    return <Navigate to="/unauthorized" replace />;
  }

  return children;
}
