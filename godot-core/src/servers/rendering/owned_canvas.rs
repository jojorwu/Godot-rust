use crate::builtin::{Rid, Vector2};
use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedCanvas,
    "A RAII wrapper for a canvas RID that is owned by this type.\nThe canvas is freed when this object is dropped."
);

impl Default for OwnedCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedCanvas {
    /// Creates a new canvas and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.canvas_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().canvas_create();
        Self { rid }
    }

    /// Sets the mirroring of a canvas item.
    ///
    /// See `RenderingServer.canvas_set_item_mirroring()`.
    pub fn set_item_mirroring(&mut self, item: Rid, mirroring: Vector2) {
        RenderingServer::singleton().canvas_set_item_mirroring(self.rid, item, mirroring);
    }
}
