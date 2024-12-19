use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

bitflags! {
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
    pub struct Permission: u32 {
        const CREATE = 1;
        const READ = 1 << 1;
        const UPDATE = 1 << 2;
        const DELETE = 1 << 3;
        const ADMIN = 1 << 4;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionSet {
    permissions: HashSet<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        PermissionSet {
            permissions: HashSet::new(),
        }
    }

    pub fn add(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    pub fn contains(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn from_permissions(permissions: &[Permission]) -> Self {
        let mut set = PermissionSet::new();
        set.permissions.extend(permissions.iter().cloned());
        set
    }

    pub fn to_u32(&self) -> u32 {
        self.permissions.iter().map(|perm| perm.bits()).sum()
    }
}
