/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `TextServer` resources.

pub mod owned_font;
pub mod owned_shaped_text;

pub use owned_font::OwnedFont;
pub use owned_shaped_text::OwnedShapedText;

impl crate::classes::TextServer {
    /// Creates a new font and returns a wrapper that will free it on drop.
    pub fn create_font_owned(&mut self) -> OwnedFont {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::TextServer>();
        let rid = gd.create_font();
        OwnedFont::from_rid(rid, gd)
    }

    /// Creates a new shaped text and returns a wrapper that will free it on drop.
    pub fn create_shaped_text_owned(&mut self) -> OwnedShapedText {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::TextServer>();
        let rid = gd.create_shaped_text();
        OwnedShapedText::from_rid(rid, gd)
    }
}
