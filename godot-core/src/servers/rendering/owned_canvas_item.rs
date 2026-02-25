use crate::builtin::{Color, Rect2, Rid, Transform2D, Vector2};
use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedCanvasItem, "A RAII wrapper for a canvas item RID that is owned by this type.\nThe canvas item is freed when this object is dropped.", @default);

impl OwnedCanvasItem {
    /// Creates a new canvas item and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.canvas_item_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().canvas_item_create();
        Self { rid }
    }

    /// Sets the parent of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_parent()`.
    #[track_caller]
    pub fn set_parent(&mut self, parent: Rid) {
        RenderingServer::singleton().canvas_item_set_parent(self.rid, parent);
    }

    /// Draws a circle on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_circle()`.
    #[track_caller]
    pub fn add_circle(&mut self, position: Vector2, radius: f32, color: Color) {
        RenderingServer::singleton().canvas_item_add_circle(self.rid, position, radius, color);
    }

    /// Sets the color modulation of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_modulate()`.
    #[track_caller]
    pub fn set_modulate(&mut self, color: Color) {
        RenderingServer::singleton().canvas_item_set_modulate(self.rid, color);
    }

    /// Sets the transform of the canvas item.
    ///
    /// See `RenderingServer.canvas_item_set_transform()`.
    #[track_caller]
    pub fn set_transform(&mut self, transform: &Transform2D) {
        RenderingServer::singleton().canvas_item_set_transform(self.rid, *transform);
    }

    /// Draws a line on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_line()`.
    #[track_caller]
    pub fn add_line(&mut self, from: Vector2, to: Vector2, color: Color, width: f32) {
        RenderingServer::singleton()
            .canvas_item_add_line_ex(self.rid, from, to, color)
            .width(width)
            .done();
    }

    /// Draws a polyline on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_polyline()`.
    #[track_caller]
    pub fn add_polyline(
        &mut self,
        points: &crate::builtin::PackedVector2Array,
        colors: &crate::builtin::PackedColorArray,
        width: f32,
        antialiased: bool,
    ) {
        RenderingServer::singleton()
            .canvas_item_add_polyline_ex(self.rid, points, colors)
            .width(width)
            .antialiased(antialiased)
            .done();
    }

    /// Draws a polygon on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_polygon()`.
    #[track_caller]
    pub fn add_polygon(
        &mut self,
        points: &crate::builtin::PackedVector2Array,
        colors: &crate::builtin::PackedColorArray,
    ) {
        RenderingServer::singleton().canvas_item_add_polygon(self.rid, points, colors);
    }

    /// Draws a rectangle on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_rect()`.
    #[track_caller]
    pub fn add_rect(&mut self, rect: Rect2, color: Color) {
        RenderingServer::singleton().canvas_item_add_rect(self.rid, rect, color);
    }

    /// Draws a texture on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_texture_rect()`.
    #[track_caller]
    pub fn add_texture_rect(&mut self, rect: Rect2, texture: Rid) {
        RenderingServer::singleton().canvas_item_add_texture_rect(self.rid, rect, texture);
    }

    /// Draws a texture region on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_texture_rect_region()`.
    #[track_caller]
    pub fn add_texture_rect_region(&mut self, rect: Rect2, texture: Rid, src_rect: Rect2) {
        RenderingServer::singleton()
            .canvas_item_add_texture_rect_region(self.rid, rect, texture, src_rect);
    }

    /// Draws a MSDF texture region on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_msdf_texture_rect_region()`.
    #[track_caller]
    pub fn add_msdf_texture_rect_region(&mut self, rect: Rect2, texture: Rid, src_rect: Rect2) {
        RenderingServer::singleton()
            .canvas_item_add_msdf_texture_rect_region(self.rid, rect, texture, src_rect);
    }

    /// Draws a mesh on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_mesh()`.
    #[track_caller]
    pub fn add_mesh(&mut self, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid) {
        RenderingServer::singleton()
            .canvas_item_add_mesh_ex(self.rid, mesh)
            .transform(transform)
            .modulate(modulate)
            .texture(texture)
            .done();
    }

    /// Draws a multimesh on the canvas item.
    ///
    /// See `RenderingServer.canvas_item_add_multimesh()`.
    #[track_caller]
    pub fn add_multimesh(&mut self, multimesh: Rid, texture: Rid) {
        RenderingServer::singleton()
            .canvas_item_add_multimesh_ex(self.rid, multimesh)
            .texture(texture)
            .done();
    }
}
