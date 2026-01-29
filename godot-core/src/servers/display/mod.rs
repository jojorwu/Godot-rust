/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `DisplayServer` resources.

pub mod owned_accessibility_element;

pub use owned_accessibility_element::OwnedAccessibilityElement;

impl crate::classes::DisplayServer {
    /// Creates a new accessibility element and returns a wrapper that will free it on drop.
    pub fn accessibility_create_element_owned(
        &mut self,
        window_id: i32,
        role: crate::classes::display_server::AccessibilityRole,
    ) -> OwnedAccessibilityElement {
        OwnedAccessibilityElement::new(window_id, role)
    }
}
