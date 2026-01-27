use crate::builtin::{Color, Rect2, Rid, Transform2D, Vector2};
use crate::classes::RenderingServer;
use crate::obj::Singleton;

/// A RAII wrapper for a canvas item RID that is owned by this type.
/// The canvas item is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedCanvasItem {
    rid: Rid,
}

impl Default for OwnedCanvasItem {
    fn default() -> Self {
        Self::new()
    }
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
        RenderingServer::singleton().canvas_item_set_transform(self.rid, *transform);
    }

    /// Draws a line on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_line()`.
    pub fn add_line(&mut self, from: Vector2, to: Vector2, color: Color, width: f32) {
        RenderingServer::singleton()
            .canvas_item_add_line_ex(self.rid, from, to, color)
            .width(width)
            .done();
    }

    /// Draws a rectangle on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_rect()`.
    pub fn add_rect(&mut self, rect: Rect2, color: Color) {
        RenderingServer::singleton().canvas_item_add_rect(self.rid, rect, color);
    }

    /// Draws a texture on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_texture_rect()`.
    pub fn add_texture_rect(&mut self, rect: Rect2, texture: Rid) {
        RenderingServer::singleton().canvas_item_add_texture_rect(self.rid, rect, texture);
    }

    /// Draws a texture region on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_texture_rect_region()`.
    pub fn add_texture_rect_region(&mut self, rect: Rect2, texture: Rid, src_rect: Rect2) {
        RenderingServer::singleton()
            .canvas_item_add_texture_rect_region(self.rid, rect, texture, src_rect);
    }

    /// Draws a MSDF texture region on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_msdf_texture_rect_region()`.
    pub fn add_msdf_texture_rect_region(&mut self, rect: Rect2, texture: Rid, src_rect: Rect2) {
        RenderingServer::singleton()
            .canvas_item_add_msdf_texture_rect_region(self.rid, rect, texture, src_rect);
    }
}

impl Drop for OwnedCanvasItem {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
