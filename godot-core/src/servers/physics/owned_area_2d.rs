/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedArea2D,
    PhysicsServer2D,
    "A RAII wrapper for a 2D physics area RID that is owned by this type.\nThe area is freed when this object is dropped.",
    @default
);

impl OwnedArea2D {
    /// Creates a new area and returns a wrapper that will free it on drop.
    ///
    /// See `PhysicsServer2D.area_create()`.
    pub fn new() -> Self {
        let rid = PhysicsServer2D::singleton().area_create();
        Self { rid }
    }
}
