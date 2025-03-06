use std::collections::HashSet;

pub mod error;

// 

/// Define the `VLMPermissions` trait.
pub trait VLMPermissions {
    fn has_permission(&self, from: &'static str, to: &'static str) -> bool;
    fn set_permission(&mut self, from: &'static str, to: &'static str, can_interact: bool);
    fn get_permissions(&self, role: &'static str) -> Option<HashSet<&'static str>>;
    fn set_permission_with_regex(&mut self, from: &str, to: &str, can_interact: bool);
    fn check_permission_with_regex(&self, from: &str, to: &str) -> bool;
}