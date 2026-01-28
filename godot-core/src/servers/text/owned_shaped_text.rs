/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::TextServer;
use crate::obj::Gd;

crate::obj::impl_owned_rid!(
    OwnedShapedText,
    TextServer,
    instance,
    "A RAII wrapper for a text server shaped text RID that is owned by this type.\nThe shaped text is freed when this object is dropped."
);

impl OwnedShapedText {
    /// Creates a new shaped text and returns a wrapper that will free it on drop.
    ///
    /// See `TextServer.create_shaped_text()`.
    pub fn new(server: &Gd<TextServer>) -> Self {
        let mut server = server.clone();
        let rid = server.create_shaped_text();
        Self { rid, server }
    }

    pub(crate) fn from_rid(rid: crate::builtin::Rid, server: Gd<TextServer>) -> Self {
        Self { rid, server }
    }
}
