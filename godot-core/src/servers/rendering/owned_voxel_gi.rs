use crate::classes::RenderingServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(OwnedVoxelGI, "A RAII wrapper for a voxel GI RID that is owned by this type.\nThe voxel GI is freed when this object is dropped.", @default);

impl OwnedVoxelGI {
    /// Creates a new voxel GI and returns a wrapper that will free it on drop.
    ///
    /// See `RenderingServer.voxel_gi_create()`.
    pub fn new() -> Self {
        let rid = RenderingServer::singleton().voxel_gi_create();
        Self { rid }
    }
}
