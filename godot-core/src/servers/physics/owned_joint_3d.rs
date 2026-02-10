/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::PhysicsServer3D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedJoint3D,
    PhysicsServer3D,
    "A RAII wrapper for a 3D physics joint RID that is owned by this type.\nThe joint is freed when this object is dropped."
);

impl OwnedJoint3D {
    /// Creates a new joint.
    pub fn new() -> Self {
        let mut server = PhysicsServer3D::singleton();
        let rid = server.joint_create();
        unsafe { Self::from_rid(rid) }
    }
}

impl Default for OwnedJoint3D {
    fn default() -> Self {
        Self::new()
    }
}
