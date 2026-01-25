
use crate::builtin::rid::Rid;
use crate::classes::RenderingServer;

/// A RAII wrapper for a texture RID that is owned by this type.
/// The texture is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedTexture {
    rid: Rid,
}

impl OwnedTexture {
    /// Creates a new placeholder texture and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.texture_2d_placeholder_create()`.
    pub fn new_placeholder() -> Self {
        let rid = RenderingServer::singleton().texture_2d_placeholder_create();
        Self { rid }
    }

    /// Returns the underlying RID of the texture.
    pub fn rid(&self) -> Rid {
        self.rid
    }
}

impl Drop for OwnedTexture {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
