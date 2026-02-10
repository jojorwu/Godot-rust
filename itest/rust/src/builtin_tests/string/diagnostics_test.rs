/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::{expect_panic, itest};
use godot::builtin::Variant;
use godot::classes::{Node, Object};
use godot::obj::{Gd, NewAlloc};

#[itest]
fn object_get_as_diagnostics() {
    let node = Node::new_alloc();

    expect_panic("Object::get_as() missing property", || {
        let _: i64 = node.get_as("non_existent_property");
    });

    // Test conversion failure diagnostic
    // 'name' is a GString property
    expect_panic("Object::get_as() conversion failure", || {
        let _: i64 = node.get_as("name");
    });

    node.free();
}

#[itest]
fn variant_operator_diagnostics() {
    let a = Variant::from(42);
    let b = Variant::from("hello");

    expect_panic("Variant operator + failure", || {
        let _ = a.clone() + b.clone();
    });

    expect_panic("Variant operator - failure", || {
        let _ = a.clone() - b.clone();
    });
}

#[itest]
fn gd_cast_diagnostics() {
    let node = Node::new_alloc();
    let obj: Gd<Object> = node.clone().upcast();

    // This will fail since Node is not a Resource
    expect_panic("Gd::cast failure diagnostic", || {
        let _ = obj.cast::<godot::classes::Resource>();
    });

    node.free();
}
