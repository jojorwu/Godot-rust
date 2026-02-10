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
    pub fn get_native_handle(&self) -> u64 {
        self.server.clone().texture_get_native_handle(self.rid)
    }
}
