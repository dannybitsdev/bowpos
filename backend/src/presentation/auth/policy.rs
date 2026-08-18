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
        &[Permission::ManageTenantAdmins]
    }
}

pub struct TenantAdminOnly;

impl AccessPolicy for TenantAdminOnly {
    fn required_roles() -> &'static [Role] {
        &[Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::ManageTenantUsers]
    }
}

pub struct MenuReadAccess;

impl AccessPolicy for MenuReadAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::CAJERO, Role::MESERO]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::ReadTenantData]
    }
}

pub struct MenuWriteAccess;

impl AccessPolicy for MenuWriteAccess {
    fn required_roles() -> &'static [Role] {
        &[Role::SUPER_ADMIN, Role::ADMIN_TENANT]
    }

    fn required_permissions() -> &'static [Permission] {
        &[Permission::ManageTenantUsers]
    }
}

pub struct OrderReadAccess;

impl AccessPolicy for OrderReadAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::CAJERO, Role::MESERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::ReadTenantData] }
}

pub struct OrderWriteAccess;

impl AccessPolicy for OrderWriteAccess {
    fn required_roles() -> &'static [Role] { &[Role::SUPER_ADMIN, Role::ADMIN_TENANT, Role::CAJERO, Role::MESERO] }
    fn required_permissions() -> &'static [Permission] { &[Permission::ReadTenantData] }
}
