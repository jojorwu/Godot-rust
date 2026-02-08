/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::builtin::{GString, Variant};
use godot::classes::Object;
use godot::obj::NewAlloc;
use godot::prelude::ToGodot;
use crate::framework::{expect_panic, itest};

#[itest]
fn variant_operator_diagnostics() {
    let a = Variant::from(1);
    let b = Variant::from("hello");

    expect_panic("Variant operator + failed between Int and String", move || {
        let _ = a + b;
    });

    expect_panic("Variant unary operator - failed for String", move || {
        let _ = -Variant::from("hello");
    });
}

#[itest]
fn object_extension_diagnostics() {
    let mut obj = Object::new_alloc();
    obj.set("prop", &42_i64.to_variant());

    expect_panic("Object::get_as(): property 'prop' conversion failed", || {
        let _: GString = obj.get_as("prop");
    });

    expect_panic("Object::get_as(): property 'missing' not found", || {
        let _: i64 = obj.get_as("missing");
    });

    obj.free();
}
