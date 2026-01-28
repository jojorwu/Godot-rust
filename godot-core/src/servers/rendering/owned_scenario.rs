use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedScenario,
    "A RAII wrapper for a scenario RID that is owned by this type.\nThe scenario is freed when this object is dropped."
);

impl Default for OwnedScenario {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedScenario {
    /// Creates a new scenario and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.scenario_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().scenario_create();
        Self { rid }
    }
}
