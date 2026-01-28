use crate::builtin::{
    Array, PackedVector2Array, PackedVector3Array, Rid, VarDictionary, VariantType,
};
use crate::classes::rendering_server::PrimitiveType;
use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedMesh,
    "A RAII wrapper for a mesh RID that is owned by this type.\nThe mesh is freed when this object is dropped."
);

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

    /// Creates a new mesh from surfaces and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.mesh_create_from_surfaces()`.
    pub fn new_from_surfaces(surfaces: &Array<VarDictionary>) -> Self {
        let rid = RenderingServer::singleton().mesh_create_from_surfaces(surfaces);
        Self { rid }
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
        let vertex_array = arrays.at(0);
        match vertex_array.get_type() {
            VariantType::PACKED_VECTOR3_ARRAY => {
                vertex_array.to::<PackedVector3Array>().len() as i32
            }
            VariantType::PACKED_VECTOR2_ARRAY => {
                vertex_array.to::<PackedVector2Array>().len() as i32
            }
            VariantType::ARRAY => vertex_array.to::<crate::builtin::AnyArray>().len() as i32,
            _ => 0,
        }
    }

    /// Returns the material of a surface.
    ///
    /// See `RenderingServer.mesh_surface_get_material()`.
    pub fn surface_get_material(&self, surface_idx: i32) -> Rid {
        RenderingServer::singleton().mesh_surface_get_material(self.rid, surface_idx)
    }

    /// Sets the material of a surface.
    ///
    /// See `RenderingServer.mesh_surface_set_material()`.
    pub fn surface_set_material(&mut self, surface_idx: i32, material: Rid) {
        RenderingServer::singleton().mesh_surface_set_material(self.rid, surface_idx, material);
    }

    /// Removes all surfaces from the mesh.
    ///
    /// See `RenderingServer.mesh_clear()`.
    pub fn clear(&mut self) {
        RenderingServer::singleton().mesh_clear(self.rid);
    }
}
