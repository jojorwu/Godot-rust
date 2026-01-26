
use crate::builtin::rid::Rid;
use crate::classes::RenderingServer;

/// A RAII wrapper for a canvas item RID that is owned by this type.
/// The canvas item is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedCanvasItem {
    rid: Rid,
}

impl OwnedCanvasItem {
    /// Creates a new canvas item and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.canvas_item_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().canvas_item_create();
        Self { rid }
    }

    /// Returns the underlying RID of the canvas item.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets the parent of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_parent()`.
    pub fn set_parent(&mut self, parent: Rid) {
        RenderingServer::singleton().canvas_item_set_parent(self.rid, parent);
    }
}

impl Drop for OwnedCanvasItem {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
