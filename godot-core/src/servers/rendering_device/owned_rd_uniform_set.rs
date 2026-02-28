/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

crate::obj::impl_owned_rid!(
    OwnedRdUniformSet,
    RenderingDevice,
    instance,
    "A RAII wrapper for a rendering device uniform set RID.\nThe uniform set is freed when this object is dropped."
);

impl OwnedRdUniformSet {
    /// Creates a new uniform set.
    #[track_caller]
    pub fn new(
        mut rd: crate::obj::Gd<crate::classes::RenderingDevice>,
        uniforms: crate::builtin::Array<crate::obj::Gd<crate::classes::RDUniform>>,
        shader: crate::builtin::Rid,
        shader_set: u32,
    ) -> Self {
        let rid = rd.uniform_set_create(&uniforms, shader, shader_set);
        Self {
            rid,
            server: rd,
            _dummy: (),
        }
    }
}
