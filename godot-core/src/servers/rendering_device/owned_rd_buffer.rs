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
    ///
    /// This method copies the data into a new Godot `PackedByteArray`. If you already have a `PackedByteArray`,
    /// use [`update_data_packed()`][Self::update_data_packed] instead to avoid the copy.
    #[track_caller]
    pub fn update_data(&mut self, data: &[u8], offset: u32) {
        let mut rd = self.server.clone();
        let packed = crate::builtin::PackedByteArray::from(data);
        rd.buffer_update(
            self.rid,
            offset,
            crate::builtin::to_u32(data.len() as u64),
            &packed,
        );
    }

    /// Updates the buffer from a Godot `PackedByteArray` at the specified offset.
    ///
    /// Use this method to avoid redundant copies if the data is already in Godot memory.
    #[track_caller]
    pub fn update_data_packed(&mut self, data: &crate::builtin::PackedByteArray, offset: u32) {
        let mut rd = self.server.clone();
        rd.buffer_update(
            self.rid,
            offset,
            crate::builtin::to_u32(data.len() as u64),
            data,
        );
    }

    /// Returns the buffer data.
    #[track_caller]
    pub fn get_data(&self) -> crate::builtin::PackedByteArray {
        let mut rd = self.server.clone();
        rd.buffer_get_data(self.rid)
    }

    /// Clears the buffer data (sets to zero).
    #[track_caller]
    pub fn clear(&mut self) {
        let mut rd = self.server.clone();
        rd.buffer_clear(self.rid, 0, crate::builtin::to_u32(rd.buffer_get_data(self.rid).len() as u64));
    }
}
