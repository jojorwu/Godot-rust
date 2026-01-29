use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedShader,
    "A RAII wrapper for a shader RID that is owned by this type.\nThe shader is freed when this object is dropped."
);

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

    /// Sets the code for the shader.
    ///
    /// See `RenderingServer.shader_set_code()`.
    pub fn set_code(&mut self, code: &str) {
        RenderingServer::singleton().shader_set_code(self.rid, code);
    }
}
