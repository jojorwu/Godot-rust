/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::NavigationServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedAgent2D,
    NavigationServer2D,
    "A RAII wrapper for a 2D navigation agent RID that is owned by this type.\nThe agent is freed when this object is dropped."
);

impl Default for OwnedAgent2D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAgent2D {
    /// Creates a new navigation agent and returns a wrapper that will free it on drop.
    ///
    /// See `NavigationServer2D.agent_create()`.
    pub fn new() -> Self {
        let rid = NavigationServer2D::singleton().agent_create();
        Self { rid }
    }
}
