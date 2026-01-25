
use crate::builtin::rid::Rid;
use crate::builtin::Dictionary;
use crate::classes::rendering_server::PrimitiveType;
use crate::classes::RenderingServer;

/// A RAII wrapper for a mesh RID that is owned by this type.
/// The mesh is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedMesh {
    rid: Rid,
}

impl OwnedMesh {
    /// Creates a new mesh and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.mesh_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().mesh_create();
        Self { rid }
    }

    /// Returns the underlying RID of the mesh.
    pub fn rid(&self) -> Rid {
        self.rid
    }

    /// Adds a surface to the mesh.
    ///
    /// See `RenderingServer.mesh_add_surface_from_arrays()`.
    pub fn add_surface(&mut self, primitive: PrimitiveType, arrays: &Dictionary) {
        RenderingServer::singleton().mesh_add_surface_from_arrays(self.rid, primitive, arrays.clone());
    }
}

impl Drop for OwnedMesh {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
