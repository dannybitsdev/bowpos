import type { PropsWithChildren, ReactNode } from 'react';

import type { Permission } from '../../domain/authTypes';
import { useAuthStore } from '../../application/authStore';

type CanProps = PropsWithChildren<{
  permissions: Permission[];
  requireAll?: boolean;
  fallback?: ReactNode;
}>;

export function usePermission(permission: Permission): boolean {
  return useAuthStore((state) => state.user?.permissions.includes(permission) ?? false);
}

export function Can({ permissions, requireAll = true, fallback = null, children }: CanProps) {
  const granted = useAuthStore((state) => {
    const assigned = state.user?.permissions ?? [];
    return requireAll
      ? permissions.every((permission) => assigned.includes(permission))
      : permissions.some((permission) => assigned.includes(permission));
  });

  return granted ? <>{children}</> : <>{fallback}</>;
}