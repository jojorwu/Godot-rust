/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

crate::obj::impl_owned_rid!(
    OwnedRdTexture,
    RenderingDevice,
    instance,
    "A RAII wrapper for a rendering device texture RID.\nThe texture is freed when this object is dropped."
);

impl OwnedRdTexture {
    /// Returns the native handle of the texture (e.g. VkImage or ID3D12Resource pointer).
    #[track_caller]
    pub fn get_native_handle(&self) -> u64 {
        self.server.clone().texture_get_native_handle(self.rid)
    }

    /// Returns the texture data.
    #[track_caller]
    pub fn get_data(&self, layer: u32) -> crate::builtin::PackedByteArray {
        let mut rd = self.server.clone();
        rd.texture_get_data(self.rid, layer)
    }

    /// Updates the texture with the given data.
    ///
    /// This method copies the data into a new Godot `PackedByteArray`. If you already have a `PackedByteArray`,
    /// use [`update_data_packed()`][Self::update_data_packed] instead to avoid the copy.
    #[track_caller]
    pub fn update_data(&mut self, layer: u32, data: &[u8]) {
        let mut rd = self.server.clone();
        let packed = crate::builtin::PackedByteArray::from(data);
        rd.texture_update(self.rid, layer, &packed);
    }

    /// Updates the texture from a Godot `PackedByteArray`.
    ///
    /// Use this method to avoid redundant copies if the data is already in Godot memory.
    #[track_caller]
    pub fn update_data_packed(&mut self, layer: u32, data: &crate::builtin::PackedByteArray) {
        let mut rd = self.server.clone();
        rd.texture_update(self.rid, layer, data);
    }
}
