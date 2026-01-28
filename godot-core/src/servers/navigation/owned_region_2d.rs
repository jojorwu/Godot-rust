/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::NavigationServer2D;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedRegion2D,
    NavigationServer2D,
    "A RAII wrapper for a 2D navigation region RID that is owned by this type.\nThe region is freed when this object is dropped."
);

impl Default for OwnedRegion2D {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedRegion2D {
    /// Creates a new navigation region and returns a wrapper that will free it on drop.
    ///
    /// See `NavigationServer2D.region_create()`.
    pub fn new() -> Self {
        let rid = NavigationServer2D::singleton().region_create();
        Self { rid }
    }
}
