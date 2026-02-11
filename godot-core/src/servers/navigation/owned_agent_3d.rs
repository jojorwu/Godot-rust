/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::NavigationServer3D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedAgent3D,
    NavigationServer3D,
    "A RAII wrapper for a 3D navigation agent RID that is owned by this type.\nThe agent is freed when this object is dropped.",
    @default
);

impl OwnedAgent3D {
    /// Creates a new navigation agent and returns a wrapper that will free it on drop.
    ///
    /// See `NavigationServer3D.agent_create()`.
    pub fn new() -> Self {
        let rid = NavigationServer3D::singleton().agent_create();
        Self { rid }
    }
}
