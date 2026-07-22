use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{email::Email, password_hash::PasswordHash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    SUPER_ADMIN,
    ADMIN_TENANT,
    CAJERO,
    MESERO,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Role::SUPER_ADMIN => "SUPER_ADMIN",
            Role::ADMIN_TENANT => "ADMIN_TENANT",
            Role::CAJERO => "CAJERO",
            Role::MESERO => "MESERO",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "SUPER_ADMIN" => Some(Role::SUPER_ADMIN),
            "ADMIN_TENANT" => Some(Role::ADMIN_TENANT),
            "CAJERO" => Some(Role::CAJERO),
            "MESERO" => Some(Role::MESERO),
            _ => None,
        }
    }

    pub fn permissions(self) -> Vec<Permission> {
        match self {
            Role::SUPER_ADMIN => vec![
                Permission::ManageGlobalAdmins,
                Permission::ManageTenantAdmins,
                Permission::ManageTenantUsers,
                Permission::ReadTenantData,
            ],
            Role::ADMIN_TENANT => vec![Permission::ManageTenantUsers, Permission::ReadTenantData],
            Role::CAJERO => vec![Permission::ProcessPayments, Permission::ReadTenantData],
            Role::MESERO => vec![Permission::CreateOrders, Permission::ReadTenantData],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    ManageGlobalAdmins,
    ManageTenantAdmins,
    ManageTenantUsers,
    ReadTenantData,
    ProcessPayments,
    CreateOrders,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Permission::ManageGlobalAdmins => "manage:global_admins",
            Permission::ManageTenantAdmins => "manage:tenant_admins",
            Permission::ManageTenantUsers => "manage:tenant_users",
            Permission::ReadTenantData => "read:tenant_data",
            Permission::ProcessPayments => "process:payments",
            Permission::CreateOrders => "create:orders",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "manage:global_admins" => Some(Permission::ManageGlobalAdmins),
            "manage:tenant_admins" => Some(Permission::ManageTenantAdmins),
            "manage:tenant_users" => Some(Permission::ManageTenantUsers),
            "read:tenant_data" => Some(Permission::ReadTenantData),
            "process:payments" => Some(Permission::ProcessPayments),
            "create:orders" => Some(Permission::CreateOrders),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub role: Role,
}

impl User {
    pub fn can_create_role(&self, target_role: Role, target_tenant_id: Uuid) -> bool {
        match self.role {
            Role::SUPER_ADMIN => target_role == Role::ADMIN_TENANT,
            Role::ADMIN_TENANT => {
                self.tenant_id == target_tenant_id
                    && matches!(target_role, Role::CAJERO | Role::MESERO)
            }
            Role::CAJERO | Role::MESERO => false,
        }
    }
}
