use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedTexture,
    "A RAII wrapper for a texture RID that is owned by this type.\nThe texture is freed when this object is dropped."
);

impl OwnedTexture {
    /// Creates a new texture from an image and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.texture_2d_create()`.
    #[cfg(feature = "codegen-full")]
    #[track_caller]
    pub fn new(image: &crate::obj::Gd<crate::classes::Image>) -> Self {
        let rid = RenderingServer::singleton().texture_2d_create(Some(image));
        Self { rid }
    }

    /// Creates a new placeholder texture and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.texture_2d_placeholder_create()`.
    #[track_caller]
    pub fn new_placeholder() -> Self {
        let rid = RenderingServer::singleton().texture_2d_placeholder_create();
        Self { rid }
    }

    /// Creates a new proxy texture from a `RenderingDevice` texture and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.texture_rd_create()`.
    #[track_caller]
    pub fn new_from_rd(rd_texture: crate::builtin::Rid) -> Self {
        let rid = RenderingServer::singleton().texture_rd_create(rd_texture);
        Self { rid }
    }
}
