use crate::builtin::{Array, Rid};
use crate::classes::rendering_server::PrimitiveType;
use crate::classes::RenderingServer;
use crate::obj::Singleton;

/// A RAII wrapper for a mesh RID that is owned by this type.
/// The mesh is freed when this object is dropped.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct OwnedMesh {
    rid: Rid,
}

impl Default for OwnedMesh {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn add_surface(
        &mut self,
        primitive: PrimitiveType,
        arrays: &Array<crate::builtin::Variant>,
    ) {
        RenderingServer::singleton().mesh_add_surface_from_arrays(self.rid, primitive, arrays);
    }

    /// Returns the number of surfaces in the mesh.
    ///
    /// See `RenderingServer.mesh_get_surface_count()`.
    pub fn get_surface_count(&self) -> i32 {
        RenderingServer::singleton().mesh_get_surface_count(self.rid)
    }

    /// Returns the number of vertices in a surface.
    ///
    /// See `RenderingServer.mesh_surface_get_arrays()`.
    pub fn surface_get_array_len(&self, surface_idx: i32) -> i32 {
        let arrays = RenderingServer::singleton().mesh_surface_get_arrays(self.rid, surface_idx);
        if arrays.is_empty() {
            return 0;
        }
        let vertex_array: crate::builtin::AnyArray = arrays.at(0).to();
        vertex_array.len() as i32
    }

    /// Removes all surfaces from the mesh.
    ///
    /// See `RenderingServer.mesh_clear()`.
    pub fn clear(&mut self) {
        RenderingServer::singleton().mesh_clear(self.rid);
    }
}

impl Drop for OwnedMesh {
    fn drop(&mut self) {
        if self.rid.is_valid() {
            RenderingServer::singleton().free_rid(self.rid);
        }
    }
}
