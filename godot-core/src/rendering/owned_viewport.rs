
use crate::builtin::rid::Rid;
use crate::builtin::Vector2i;
use crate::classes::RenderingServer;

/// A RAII wrapper for a viewport RID that is owned by this type.
/// The viewport is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedViewport {
    rid: Rid,
}

impl OwnedViewport {
    /// Creates a new viewport and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.viewport_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().viewport_create();
        Self { rid }
    }

    /// Returns the underlying RID of the viewport.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets the size of the viewport.
    ///
    /// See `RenderingServer.viewport_set_size()`.
    pub fn set_size(&mut self, width: i32, height: i32) {
        RenderingServer::singleton().viewport_set_size(self.rid, width, height);
    }
}

impl Drop for OwnedViewport {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
