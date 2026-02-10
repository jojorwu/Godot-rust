/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::{expect_panic, itest};
use godot::builtin::GString;
use godot::classes::{Node, Object};
use godot::obj::NewAlloc;
use godot::prelude::ToGodot;

#[itest]
fn object_get_as_reliability() {
    let mut obj = Object::new_alloc();
    obj.set("my_prop", &42_i64.to_variant());

    // Valid case
    let val: i64 = obj.get_as("my_prop");
    assert_eq!(val, 42);

    // Missing property
    expect_panic(
        "Object::get_as(): property 'non_existent' not found",
        || {
            let _: i64 = obj.get_as("non_existent");
        },
    );

    // Wrong type
    expect_panic(
        "cannot be converted to godot::builtin::strings::gstring::GString",
        || {
            let _: GString = obj.get_as("my_prop");
        },
    );

    obj.free();
}

#[itest]
fn object_meta_as_reliability() {
    let mut obj = Object::new_alloc();
    obj.set_meta("my_meta", &"hello".to_variant());

    // Valid case
    let val: GString = obj.get_meta_as("my_meta");
    assert_eq!(val, "hello");

    // Missing meta
    expect_panic(
        "Object::get_meta_as(): meta 'non_existent' not found",
        || {
            let _: GString = obj.get_meta_as("non_existent");
        },
    );

    // Wrong type
    expect_panic("cannot be converted to i64", || {
        let _: i64 = obj.get_meta_as("my_meta");
    });

    obj.free();
}

#[itest]
fn node_get_tree_as_reliability() {
    let node = Node::new_alloc();

    // Node not in tree
    expect_panic("Node::get_tree_as(): node is not in the scene tree", || {
        node.get_tree_as::<godot::classes::SceneTree>();
    });

    // try_get_tree_as returns None
    assert!(node
        .try_get_tree_as::<godot::classes::SceneTree>()
        .is_none());

    node.free();
}
