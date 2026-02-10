/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::DisplayServer;
use crate::obj::Singleton;

crate::obj::impl_owned_rid!(
    OwnedAccessibilityElement,
    DisplayServer,
    "A RAII wrapper for a DisplayServer accessibility element RID that is owned by this type.\nThe element is freed when this object is dropped.",
    accessibility_free_element
);

impl OwnedAccessibilityElement {
    /// Creates a new accessibility element.
    pub fn new(window_id: i32, role: crate::classes::display_server::AccessibilityRole) -> Self {
        let mut server = DisplayServer::singleton();
        let rid = server.accessibility_create_element(window_id, role);
        unsafe { Self::from_rid(rid) }
    }
}
