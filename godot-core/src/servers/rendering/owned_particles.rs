use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedParticles,
    "A RAII wrapper for a particles RID that is owned by this type.\nThe particles are freed when this object is dropped.",
    @default
);

impl OwnedParticles {
    /// Creates a new particles and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.particles_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().particles_create();
        Self { rid }
    }
}
