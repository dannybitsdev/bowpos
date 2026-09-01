use crate::domain::auth::{Permission, Role};

pub trait AccessPolicy: Send + Sync + 'static {
    fn required_roles() -> &'static [Role];
    fn required_permissions() -> &'static [Permission];
}

pub struct DenyByDefault;

impl AccessPolicy for DenyByDefault {
    fn required_roles() -> &'static [Role] {
        &[]
    }

    fn required_permissions() -> &'static [Permission] {
        &[]
    }
}

pub struct SuperAdminOnly;

impl AccessPolicy for SuperAdminOnly {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::UsersCreate]
    }
}

pub struct TenantAdminOnly;

impl AccessPolicy for TenantAdminOnly {
    fn required_roles() -> &'static [Role] {
        &[Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::UsersCreate]
    }
}

pub struct MenuReadAccess;

impl AccessPolicy for MenuReadAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER, Role::CAJERO, Role::MESERO]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::InventoryRead]
    }
}

pub struct MenuWriteAccess;

impl AccessPolicy for MenuWriteAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::InventoryAdmin]
    }
}

/// Un BRANCH_MANAGER solo puede fijar overrides (precio/stock/disponibilidad) de su propia sede.
pub struct BranchCatalogWriteAccess;

impl AccessPolicy for BranchCatalogWriteAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::InventoryUpdate]
    }
}

/// Crear/listar sedes es exclusivo de administración del tenant o superior.
pub struct LocationWriteAccess;

impl AccessPolicy for LocationWriteAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::ConfigLocations]
    }
}

/// Asignar usuarios a sedes: tenant admin o superior.
pub struct BranchAssignmentAccess;

impl AccessPolicy for BranchAssignmentAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::UsersCreate]
    }
}

pub struct OrderReadAccess;

impl AccessPolicy for OrderReadAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER, Role::CAJERO, Role::MESERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::OrdersRead] }
}

pub struct OrderWriteAccess;

impl AccessPolicy for OrderWriteAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER, Role::CAJERO, Role::MESERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::OrdersCreate] }
}

pub struct OrderStatusUpdateAccess;

impl AccessPolicy for OrderStatusUpdateAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER, Role::MESERO, Role::COCINERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::OrdersUpdate] }
}

pub struct SalesReadAccess;

impl AccessPolicy for SalesReadAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::BRANCH_MANAGER, Role::CAJERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::SalesRead] }
}

pub struct ConfigUpdateAccess;

impl AccessPolicy for ConfigUpdateAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT] }
    fn required_permissions() -> &'static [Permission] { &[Permission::ConfigUpdate] }
}
