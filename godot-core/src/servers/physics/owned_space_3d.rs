/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer3D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedSpace3D,
    PhysicsServer3D,
    "A RAII wrapper for a 3D physics space RID that is owned by this type.\nThe space is freed when this object is dropped."
);

impl Default for OwnedSpace3D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedSpace3D {
    /// Creates a new space and returns a wrapper that will free it on drop.
    ///
    /// See `PhysicsServer3D.space_create()`.
    pub fn new() -> Self {
        let rid = PhysicsServer3D::singleton().space_create();
        Self { rid }
    }
}
