use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedViewport,
    "A RAII wrapper for a viewport RID that is owned by this type.\nThe viewport is freed when this object is dropped."
);

impl Default for OwnedViewport {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedViewport {
    /// Creates a new viewport and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.viewport_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().viewport_create();
        Self { rid }
    }

    /// Sets the size of the viewport.
    ///
    /// See `RenderingServer.viewport_set_size()`.
    pub fn set_size(&mut self, width: i32, height: i32) {
        RenderingServer::singleton().viewport_set_size(self.rid, width, height);
    }
}
