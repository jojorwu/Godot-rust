/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{Array, PackedInt64Array, Rid};
use crate::classes::RenderingDevice;
use crate::meta::GodotType;

crate::obj::impl_owned_rid!(
    OwnedRdVertexArray,
    RenderingDevice,
    instance,
    "A RAII wrapper for a RenderingDevice vertex array RID that is owned by this type.\nThe array is freed when this object is dropped."
);

impl OwnedRdVertexArray {
    /// Creates a new vertex array.
    pub fn new(
        server: &crate::obj::Gd<RenderingDevice>,
        vertex_count: u32,
        vertex_format: i64,
        src_buffers: &Array<Rid>,
    ) -> Self {
        let mut server = server.clone();
        let rid = server.vertex_array_create(vertex_count, vertex_format, src_buffers);
        unsafe { Self::from_rid(rid, server) }
    }

    /// Creates a new vertex array with offsets.
    pub fn new_with_offsets(
        server: &crate::obj::Gd<RenderingDevice>,
        vertex_count: u32,
        vertex_format: i64,
        src_buffers: &Array<Rid>,
        offsets: &PackedInt64Array,
    ) -> Self {
        let mut server = server.clone();
        let rid = server.vertex_array_create_full(
            vertex_count,
            vertex_format,
            src_buffers.to_ffi(),
            offsets.to_ffi(),
        );
        unsafe { Self::from_rid(rid, server) }
    }
}
