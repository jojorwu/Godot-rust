
use crate::builtin::rid::Rid;
use crate::builtin::{Color, Transform2D, Vector2};
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

    /// Draws a circle on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_circle()`.
    pub fn add_circle(&mut self, position: Vector2, radius: f32, color: Color) {
        RenderingServer::singleton().canvas_item_add_circle(self.rid, position, radius, color);
    }

    /// Sets the color modulation of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_modulate()`.
    pub fn set_modulate(&mut self, color: Color) {
        RenderingServer::singleton().canvas_item_set_modulate(self.rid, color);
    }

    /// Sets the transform of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_transform()`.
    pub fn set_transform(&mut self, transform: &Transform2D) {
        RenderingServer::singleton().canvas_item_set_transform(self.rid, transform.clone());
    }

    /// Draws a line on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_line()`.
    pub fn add_line(&mut self, from: Vector2, to: Vector2, color: Color, width: f32) {
        RenderingServer::singleton().canvas_item_add_line(self.rid, from, to, color, width);
    }

    /// Draws a rectangle on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_rect()`.
    pub fn add_rect(&mut self, rect: crate::builtin::Rect2, color: Color) {
        RenderingServer::singleton().canvas_item_add_rect(self.rid, rect, color);
    }
}

impl Drop for OwnedCanvasItem {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
