/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer2D;
use crate::obj::Singleton;

crate::physics::impl_owned_rid!(
    OwnedBody2D,
    PhysicsServer2D,
    "A RAII wrapper for a 2D physics body RID that is owned by this type.\nThe body is freed when this object is dropped."
);

impl Default for OwnedBody2D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedBody2D {
    /// Creates a new body and returns a wrapper that will free it on drop.
    ///
    /// See `PhysicsServer2D.body_create()`.
    pub fn new() -> Self {
        let rid = PhysicsServer2D::singleton().body_create();
        Self { rid }
    }
}
