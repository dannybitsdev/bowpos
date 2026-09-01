export type Role = 'SUPER_ADMIN' | 'ADMIN_TENANT' | 'BRANCH_MANAGER' | 'CAJERO' | 'MESERO' | 'COCINERO';

export type Permission =
  | 'dashboard:read'
  | 'ventas:read'
  | 'ventas:create'
  | 'ventas:cancel'
  | 'ordenes:read'
  | 'ordenes:create'
  | 'ordenes:update'
  | 'inventario:read'
  | 'inventario:update'
  | 'inventario:admin'
  | 'config:read'
  | 'config:update'
  | 'config:sedes'
  | 'usuarios:read'
  | 'usuarios:create'
  | 'usuarios:delete'
  | 'legal:read'
  | 'legal:update';

export interface AuthUser {
  user_id: string;
  tenant_id: string;
  tenant_name?: string;
  name?: string;
  role: Role;
  permissions: Permission[];
  branch_ids?: string[];
}

export interface LoginResult {
  tokens: {
    access_token: string;
    refresh_token: string;
    token_type: string;
    expires_in: number;
  };
  user: AuthUser;
}
