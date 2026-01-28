/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::TextServer;
use crate::obj::Gd;

crate::obj::impl_owned_rid!(
    OwnedFont,
    TextServer,
    instance,
    "A RAII wrapper for a text server font RID that is owned by this type.\nThe font is freed when this object is dropped."
);

impl OwnedFont {
    /// Creates a new font and returns a wrapper that will free it on drop.
    ///
    /// See `TextServer.create_font()`.
    pub fn new(server: &Gd<TextServer>) -> Self {
        let mut server = server.clone();
        let rid = server.create_font();
        Self { rid, server }
    }

    pub(crate) fn from_rid(rid: crate::builtin::Rid, server: Gd<TextServer>) -> Self {
        Self { rid, server }
    }
}
