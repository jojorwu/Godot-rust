/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

crate::obj::impl_owned_rid!(
    OwnedRdShader,
    RenderingDevice,
    instance,
    "A RAII wrapper for a rendering device shader RID.\nThe shader is freed when this object is dropped."
);

impl OwnedRdShader {
    /// Creates a new shader from SPIR-V code.
    #[track_caller]
    pub fn new_from_spirv(
        mut rd: crate::obj::Gd<crate::classes::RenderingDevice>,
        spirv: &crate::obj::Gd<crate::classes::RDShaderSPIRV>,
        name: impl crate::meta::AsArg<crate::builtin::GString>,
    ) -> Self {
        let rid = rd.shader_create_from_spirv(spirv, name);
        Self {
            rid,
            server: rd,
            _dummy: (),
        }
    }
}
