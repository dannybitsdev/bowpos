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
