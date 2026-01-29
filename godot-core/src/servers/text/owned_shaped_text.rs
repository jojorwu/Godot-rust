/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


crate::obj::impl_owned_rid!(
    OwnedShapedText,
    TextServer,
    instance,
    "A RAII wrapper for a text server shaped text RID that is owned by this type.\nThe shaped text is freed when this object is dropped."
);
