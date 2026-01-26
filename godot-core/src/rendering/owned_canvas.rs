
use crate::builtin::rid::Rid;
use crate::builtin::Vector2;
use crate::classes::RenderingServer;

/// A RAII wrapper for a canvas RID that is owned by this type.
/// The canvas is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedCanvas {
    rid: Rid,
}

impl OwnedCanvas {
    /// Creates a new canvas and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.canvas_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().canvas_create();
        Self { rid }
    }

    /// Returns the underlying RID of the canvas.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets the mirroring of a canvas item.
    ///
    /// See `RenderingServer.canvas_set_item_mirroring()`.
    pub fn set_item_mirroring(&mut self, item: Rid, mirroring: Vector2) {
        RenderingServer::singleton().canvas_set_item_mirroring(self.rid, item, mirroring);
    }
}

impl Drop for OwnedCanvas {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
