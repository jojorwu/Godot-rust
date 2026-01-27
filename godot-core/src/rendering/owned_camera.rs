use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::rendering::impl_owned_rid!(
    OwnedCamera,
    "A RAII wrapper for a camera RID that is owned by this type.\nThe camera is freed when this object is dropped."
);

impl Default for OwnedCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedCamera {
    /// Creates a new camera and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.camera_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().camera_create();
        Self { rid }
    }
}
