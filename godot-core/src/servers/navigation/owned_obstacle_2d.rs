/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::NavigationServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedObstacle2D,
    NavigationServer2D,
    "A RAII wrapper for a 2D navigation obstacle RID that is owned by this type.
The obstacle is freed when this object is dropped.",
    @default
);

impl OwnedObstacle2D {
    /// Creates a new navigation obstacle and returns a wrapper that will free it on drop.
    ///
    /// See `NavigationServer2D.obstacle_create()`.
    pub fn new() -> Self {
        let rid = NavigationServer2D::singleton().obstacle_create();
        Self { rid }
    }
}
