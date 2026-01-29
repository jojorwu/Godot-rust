/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


use crate::builtin::Rid;
use crate::classes::RenderingDevice;

crate::obj::impl_owned_rid!(
    OwnedRdIndexArray,
    RenderingDevice,
    instance,
    "A RAII wrapper for a RenderingDevice index array RID that is owned by this type.\nThe array is freed when this object is dropped."
);

impl OwnedRdIndexArray {
    /// Creates a new index array.
    pub fn new(
        server: &crate::obj::Gd<RenderingDevice>,
        index_buffer: Rid,
        index_offset: u32,
        index_count: u32,
    ) -> Self {
        let mut server = server.clone();
        let rid = server.index_array_create(index_buffer, index_offset, index_count);
        unsafe { Self::from_rid(rid, server) }
    }
}
