use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::permissions::{Permission, PermissionSet};

#[derive(Debug, Serialize, Deserialize, Clone, EnumString, EnumIter, Display)]
pub enum Role {
    #[strum(serialize = "admin")]
    Admin,

    #[strum(serialize = "user")]
    User,
}

impl Role {
    pub fn permissions(&self) -> PermissionSet {
        match self {
            Role::Admin => PermissionSet::from_permissions(&[
                Permission::CREATE,
                Permission::READ,
                Permission::UPDATE,
                Permission::DELETE,
                Permission::ADMIN,
            ]),

            Role::User => PermissionSet::from_permissions(&[
                Permission::CREATE,
                Permission::READ,
                Permission::UPDATE,
                Permission::DELETE,
            ]),
        }
    }

    pub fn has(&self, permission: Permission) -> bool {
        self.permissions().contains(permission)
    }
}
