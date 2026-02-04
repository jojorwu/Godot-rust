/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use godot::classes::{Node, Resource};
use crate::framework::itest;

#[itest]
fn test_object_property_accessors() {
    let mut node = Node::new_alloc();
    let name = "TestNode";

    // Test set_as and get_as
    node.set_as("name", name);
    let name_back: GString = node.get_as("name");
    assert_eq!(name_back, GString::from(name));

    // Test try_get_as
    let name_opt: Option<GString> = node.try_get_as("name");
    assert_eq!(name_opt, Some(GString::from(name)));

    let bad_opt: Option<i64> = node.try_get_as("name");
    assert_eq!(bad_opt, None);

    node.free();
}

#[itest]
fn test_object_meta_accessors() {
    let mut node = Node::new_alloc();
    let meta_key = "my_meta";
    let meta_val = 42i64;

    // Test set_meta_as and get_meta_as
    node.set_meta_as(meta_key, meta_val);
    let val_back: i64 = node.get_meta_as(meta_key);
    assert_eq!(val_back, meta_val);

    // Test try_get_meta_as
    let val_opt: Option<i64> = node.try_get_meta_as(meta_key);
    assert_eq!(val_opt, Some(meta_val));

    let bad_opt: Option<GString> = node.try_get_meta_as(meta_key);
    assert_eq!(bad_opt, None);

    node.free();
}

#[itest]
fn test_resource_duplication() {
    let res = Resource::new_gd();

    // Test duplicate_as
    let dup: Gd<Resource> = res.duplicate_as(false);
    assert!(dup.is_instance_valid());
    assert_ne!(res.instance_id(), dup.instance_id());

    // Test duplicate_typed
    let dup_typed: Gd<Resource> = res.duplicate_typed::<Resource>(false);
    assert!(dup_typed.is_instance_valid());
}
