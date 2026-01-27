use crate::builtin::{Rid, Variant};
use crate::classes::RenderingServer;
use crate::obj::Singleton;

/// A RAII wrapper for a material RID that is owned by this type.
/// The material is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedMaterial {
    rid: Rid,
}

impl Default for OwnedMaterial {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedMaterial {
    /// Creates a new material and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.material_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().material_create();
        Self { rid }
    }

    /// Returns the underlying RID of the material.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets a parameter on the material.
    ///
    /// See `RenderingServer.material_set_param()`.
    pub fn set_param(&mut self, param: &str, value: &Variant) {
        RenderingServer::singleton().material_set_param(self.rid, param, value);
    }
}

impl Drop for OwnedMaterial {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
