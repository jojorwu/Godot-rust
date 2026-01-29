/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::RenderingDevice;
use crate::obj::Gd;

crate::obj::impl_owned_rid!(
    OwnedRdUniformSet,
    RenderingDevice,
    instance,
    "A RAII wrapper for a rendering device uniform set RID.\nThe uniform set is freed when this object is dropped."
);

impl OwnedRdUniformSet {
    pub(crate) fn from_rid(rid: crate::builtin::Rid, server: Gd<RenderingDevice>) -> Self {
        Self { rid, server }
    }
}
