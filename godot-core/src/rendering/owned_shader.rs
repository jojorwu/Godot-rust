use crate::builtin::Rid;
use crate::classes::RenderingServer;
use crate::obj::Singleton;

/// A RAII wrapper for a shader RID that is owned by this type.
/// The shader is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedShader {
    rid: Rid,
}

impl Default for OwnedShader {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedShader {
    /// Creates a new shader and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.shader_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().shader_create();
        Self { rid }
    }

    /// Returns the underlying RID of the shader.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Sets the code for the shader.
    ///
    /// See `RenderingServer.shader_set_code()`.
    pub fn set_code(&mut self, code: &str) {
        RenderingServer::singleton().shader_set_code(self.rid, code);
    }
}

impl Drop for OwnedShader {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
