
use crate::builtin::rid::Rid;
use crate::builtin::Color;
use crate::classes::RenderingServer;
use crate::classes::rendering_server::LightType;

/// A RAII wrapper for a light RID that is owned by this type.
/// The light is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedLight {
    rid: Rid,
}

impl OwnedLight {
    /// Creates a new light of the given type and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.light_create()`.
    pub fn new(light_type: LightType) -> Self {
        let rid = RenderingServer::singleton().light_create(light_type);
        Self { rid }
    }

    /// Returns the underlying RID of the light.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets the color of the light.
    ///
    /// See `RenderingServer.light_set_color()`.
    pub fn set_color(&mut self, color: Color) {
        RenderingServer::singleton().light_set_color(self.rid, color);
    }
}

impl Drop for OwnedLight {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
