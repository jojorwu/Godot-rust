/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */


crate::obj::impl_owned_rid!(
    OwnedFont,
    TextServer,
    instance,
    "A RAII wrapper for a text server font RID that is owned by this type.\nThe font is freed when this object is dropped."
);
