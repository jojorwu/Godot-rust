use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedOccluder,
    "A RAII wrapper for an occluder RID that is owned by this type.\nThe occluder is freed when this object is dropped."
);

impl Default for OwnedOccluder {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedOccluder {
    /// Creates a new occluder and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.occluder_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().occluder_create();
        Self { rid }
    }
}
