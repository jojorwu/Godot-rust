use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedCameraAttributes, "A RAII wrapper for a camera attributes RID that is owned by this type.\nThe camera attributes are freed when this object is dropped.", @default);

impl OwnedCameraAttributes {
    /// Creates a new camera attributes and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.camera_attributes_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().camera_attributes_create();
        Self { rid }
    }
}
