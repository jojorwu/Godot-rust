use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedLightmap, "A RAII wrapper for a lightmap RID that is owned by this type.\nThe lightmap is freed when this object is dropped.", @default);

impl OwnedLightmap {
    /// Creates a new lightmap and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.lightmap_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().lightmap_create();
        Self { rid }
    }
}
