use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedSky, "A RAII wrapper for a sky RID that is owned by this type.\nThe sky is freed when this object is dropped.", @default);

impl OwnedSky {
    /// Creates a new sky and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.sky_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().sky_create();
        Self { rid }
    }
}
