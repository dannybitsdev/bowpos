export type Role = 'SUPER_ADMIN' | 'ADMIN_TENANT' | 'CAJERO' | 'MESERO';

export interface AuthUser {
  user_id: string;
  tenant_id: string;
  tenant_name?: string;
  name?: string;
  role: Role;
  permissions: string[];
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
