use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedEnvironment, "A RAII wrapper for an environment RID that is owned by this type.\nThe environment is freed when this object is dropped.", @default);

impl OwnedEnvironment {
    /// Creates a new environment and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.environment_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().environment_create();
        Self { rid }
    }
}
