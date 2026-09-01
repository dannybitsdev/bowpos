use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{email::Email, password_hash::PasswordHash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    SUPER_ADMIN,
    ADMIN_TENANT,
    BRANCH_MANAGER,
    CAJERO,
    MESERO,
    COCINERO,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Role::SUPER_ADMIN => "SUPER_ADMIN",
            Role::ADMIN_TENANT => "ADMIN_TENANT",
            Role::BRANCH_MANAGER => "BRANCH_MANAGER",
            Role::CAJERO => "CAJERO",
            Role::MESERO => "MESERO",
            Role::COCINERO => "COCINERO",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "SUPER_ADMIN" => Some(Role::SUPER_ADMIN),
            "ADMIN_TENANT" => Some(Role::ADMIN_TENANT),
            "BRANCH_MANAGER" => Some(Role::BRANCH_MANAGER),
            "CAJERO" => Some(Role::CAJERO),
            "MESERO" => Some(Role::MESERO),
            "COCINERO" => Some(Role::COCINERO),
            _ => None,
        }
    }

    pub fn permissions(self) -> Vec<Permission> {
        match self {
            Role::SUPER_ADMIN | Role::ADMIN_TENANT => Permission::all().to_vec(),
            Role::BRANCH_MANAGER => vec![
                Permission::DashboardRead,
                Permission::SalesRead,
                Permission::SalesCreate,
                Permission::SalesCancel,
                Permission::OrdersRead,
                Permission::OrdersCreate,
                Permission::OrdersUpdate,
                Permission::InventoryRead,
                Permission::InventoryUpdate,
                Permission::ConfigRead,
                Permission::UsersRead,
            ],
            Role::CAJERO => vec![
                Permission::DashboardRead,
                Permission::SalesRead,
                Permission::SalesCreate,
                Permission::OrdersRead,
                Permission::OrdersCreate,
            ],
            Role::MESERO => vec![
                Permission::DashboardRead,
                Permission::OrdersRead,
                Permission::OrdersCreate,
                Permission::OrdersUpdate,
            ],
            Role::COCINERO => vec![Permission::DashboardRead, Permission::OrdersRead, Permission::OrdersUpdate],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    DashboardRead,
    SalesRead,
    SalesCreate,
    SalesCancel,
    OrdersRead,
    OrdersCreate,
    OrdersUpdate,
    InventoryRead,
    InventoryUpdate,
    InventoryAdmin,
    ConfigRead,
    ConfigUpdate,
    ConfigLocations,
    UsersRead,
    UsersCreate,
    UsersDelete,
    LegalRead,
    LegalUpdate,
}

impl Permission {
    pub const fn all() -> &'static [Permission] {
        &[
            Permission::DashboardRead, Permission::SalesRead, Permission::SalesCreate,
            Permission::SalesCancel, Permission::OrdersRead, Permission::OrdersCreate,
            Permission::OrdersUpdate, Permission::InventoryRead, Permission::InventoryUpdate,
            Permission::InventoryAdmin, Permission::ConfigRead, Permission::ConfigUpdate,
            Permission::ConfigLocations, Permission::UsersRead, Permission::UsersCreate,
            Permission::UsersDelete, Permission::LegalRead, Permission::LegalUpdate,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Permission::DashboardRead => "dashboard:read",
            Permission::SalesRead => "ventas:read",
            Permission::SalesCreate => "ventas:create",
            Permission::SalesCancel => "ventas:cancel",
            Permission::OrdersRead => "ordenes:read",
            Permission::OrdersCreate => "ordenes:create",
            Permission::OrdersUpdate => "ordenes:update",
            Permission::InventoryRead => "inventario:read",
            Permission::InventoryUpdate => "inventario:update",
            Permission::InventoryAdmin => "inventario:admin",
            Permission::ConfigRead => "config:read",
            Permission::ConfigUpdate => "config:update",
            Permission::ConfigLocations => "config:sedes",
            Permission::UsersRead => "usuarios:read",
            Permission::UsersCreate => "usuarios:create",
            Permission::UsersDelete => "usuarios:delete",
            Permission::LegalRead => "legal:read",
            Permission::LegalUpdate => "legal:update",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "dashboard:read" => Some(Permission::DashboardRead),
            "ventas:read" => Some(Permission::SalesRead),
            "ventas:create" => Some(Permission::SalesCreate),
            "ventas:cancel" => Some(Permission::SalesCancel),
            "ordenes:read" => Some(Permission::OrdersRead),
            "ordenes:create" => Some(Permission::OrdersCreate),
            "ordenes:update" => Some(Permission::OrdersUpdate),
            "inventario:read" => Some(Permission::InventoryRead),
            "inventario:update" => Some(Permission::InventoryUpdate),
            "inventario:admin" => Some(Permission::InventoryAdmin),
            "config:read" => Some(Permission::ConfigRead),
            "config:update" => Some(Permission::ConfigUpdate),
            "config:sedes" => Some(Permission::ConfigLocations),
            "usuarios:read" => Some(Permission::UsersRead),
            "usuarios:create" => Some(Permission::UsersCreate),
            "usuarios:delete" => Some(Permission::UsersDelete),
            "legal:read" => Some(Permission::LegalRead),
            "legal:update" => Some(Permission::LegalUpdate),
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
    pub tenant_name: String,
    pub name: String,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub role: Role,
    /// Sedes a las que el usuario tiene acceso; vacío = todas las sedes del tenant (roles admin).
    pub branch_ids: Vec<Uuid>,
}

impl User {
    pub fn can_create_role(&self, target_role: Role, target_tenant_id: Uuid) -> bool {
        match self.role {
            Role::SUPER_ADMIN => target_role == Role::ADMIN_TENANT,
            Role::ADMIN_TENANT => {
                self.tenant_id == target_tenant_id
                    && matches!(target_role, Role::BRANCH_MANAGER | Role::CAJERO | Role::MESERO)
            }
            Role::BRANCH_MANAGER => {
                self.tenant_id == target_tenant_id
                    && matches!(target_role, Role::CAJERO | Role::MESERO)
            }
            Role::CAJERO | Role::MESERO | Role::COCINERO => false,
        }
    }

    /// Roles operativos deben operar siempre bajo una sede activa explícita.
    pub fn requires_explicit_branch(&self) -> bool {
        matches!(self.role, Role::BRANCH_MANAGER | Role::CAJERO | Role::MESERO | Role::COCINERO)
    }
}

#[cfg(test)]
mod tests {
    use super::{Permission, Role};

    #[test]
    fn cashier_cannot_cancel_sales_or_manage_inventory() {
        let permissions = Role::CAJERO.permissions();
        assert!(permissions.contains(&Permission::SalesCreate));
        assert!(!permissions.contains(&Permission::SalesCancel));
        assert!(!permissions.contains(&Permission::InventoryAdmin));
    }

    #[test]
    fn super_admin_has_every_permission() {
        assert_eq!(Role::SUPER_ADMIN.permissions(), Permission::all());
    }
}
