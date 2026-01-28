/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::NavigationServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedLink2D,
    NavigationServer2D,
    "A RAII wrapper for a 2D navigation link RID that is owned by this type.\nThe link is freed when this object is dropped."
);

impl Default for OwnedLink2D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedLink2D {
    /// Creates a new navigation link and returns a wrapper that will free it on drop.
    ///
    /// See `NavigationServer2D.link_create()`.
    pub fn new() -> Self {
        let rid = NavigationServer2D::singleton().link_create();
        Self { rid }
    }
}
