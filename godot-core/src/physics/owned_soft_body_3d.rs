/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer3D;
use crate::obj::Singleton;

crate::physics::impl_owned_rid!(
    OwnedSoftBody3D,
    PhysicsServer3D,
    "A RAII wrapper for a 3D physics soft body RID that is owned by this type.\nThe soft body is freed when this object is dropped."
);

impl Default for OwnedSoftBody3D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedSoftBody3D {
    /// Creates a new soft body and returns a wrapper that will free it on drop.
    ///
    /// See `PhysicsServer3D.soft_body_create()`.
    pub fn new() -> Self {
        let rid = PhysicsServer3D::singleton().soft_body_create();
        Self { rid }
    }
}
