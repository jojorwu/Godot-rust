use crate::builtin::Variant;
use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedMaterial, "A RAII wrapper for a material RID that is owned by this type.\nThe material is freed when this object is dropped.", @default);

impl OwnedMaterial {
    /// Creates a new material and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.material_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().material_create();
        Self { rid }
    }

    /// Sets a parameter on the material.
    ///
    /// See `RenderingServer.material_set_param()`.
    pub fn set_param(&mut self, param: &str, value: &Variant) {
        RenderingServer::singleton().material_set_param(self.rid, param, value);
    }
}
