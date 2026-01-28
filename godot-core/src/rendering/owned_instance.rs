use crate::builtin::Rid;
use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::rendering::impl_owned_rid!(
    OwnedInstance,
    "A RAII wrapper for an instance RID that is owned by this type.\nThe instance is freed when this object is dropped."
);

impl Default for OwnedInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedInstance {
    /// Creates a new instance and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.instance_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().instance_create();
        Self { rid }
    }

    /// Creates a new instance with base and scenario and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.instance_create2()`.
    pub fn new_with_base(base: Rid, scenario: Rid) -> Self {
        let rid = RenderingServer::singleton().instance_create2(base, scenario);
        Self { rid }
    }

    /// Sets the base of the instance.
    ///
    /// See `RenderingServer.instance_set_base()`.
    pub fn set_base(&mut self, base: Rid) {
        RenderingServer::singleton().instance_set_base(self.rid, base);
    }

    /// Sets the scenario of the instance.
    ///
    /// See `RenderingServer.instance_set_scenario()`.
    pub fn set_scenario(&mut self, scenario: Rid) {
        RenderingServer::singleton().instance_set_scenario(self.rid, scenario);
    }

    /// Sets the transform of the instance.
    ///
    /// See `RenderingServer.instance_set_transform()`.
    pub fn set_transform(&mut self, transform: &crate::builtin::Transform3D) {
        RenderingServer::singleton().instance_set_transform(self.rid, *transform);
    }

    /// Sets whether the instance is visible.
    ///
    /// See `RenderingServer.instance_set_visible()`.
    pub fn set_visible(&mut self, visible: bool) {
        RenderingServer::singleton().instance_set_visible(self.rid, visible);
    }
}
