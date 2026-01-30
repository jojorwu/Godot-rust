/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


crate::obj::impl_owned_rid!(
    OwnedRdBuffer,
    RenderingDevice,
    instance,
    "A RAII wrapper for a rendering device buffer RID.\nThe buffer is freed when this object is dropped."
);

impl OwnedRdBuffer {
    /// Updates the buffer with the given data at the specified offset.
    pub fn update_data(&mut self, data: &[u8], offset: u32) {
        let mut rd = self.server.clone();
        rd.buffer_update(self.rid, offset, data.len() as u32, data);
    }

    /// Returns the buffer data at the specified offset and size.
    pub fn get_data(&self, offset: u32, size: u32) -> crate::builtin::PackedByteArray {
        let mut rd = self.server.clone();
        rd.buffer_get_data(self.rid, offset, size)
    }

    /// Returns the native handle of the buffer (e.g. VkBuffer or ID3D12Resource pointer).
    pub fn get_native_handle(&self) -> u64 {
        self.server.clone().buffer_get_native_handle(self.rid)
    }
}
