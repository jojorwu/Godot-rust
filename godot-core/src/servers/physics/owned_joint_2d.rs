/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedJoint2D,
    PhysicsServer2D,
    "A RAII wrapper for a 2D physics joint RID that is owned by this type.\nThe joint is freed when this object is dropped."
);

impl OwnedJoint2D {
    /// Creates a new joint.
    pub fn new() -> Self {
        let mut server = PhysicsServer2D::singleton();
        let rid = server.joint_create();
        unsafe { Self::from_rid(rid) }
    }
}

impl Default for OwnedJoint2D {
    fn default() -> Self {
        Self::new()
    }
}
