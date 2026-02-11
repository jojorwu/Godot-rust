use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedFogVolume, "A RAII wrapper for a fog volume RID that is owned by this type.\nThe fog volume is freed when this object is dropped.", @default);

impl OwnedFogVolume {
    /// Creates a new fog volume and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.fog_volume_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().fog_volume_create();
        Self { rid }
    }
}
