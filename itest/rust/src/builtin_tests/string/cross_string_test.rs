/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::prelude::*;

#[itest]
fn test_cross_string_equality() {
    let s = GString::from("hello");
    let sn = StringName::from("hello");
    let np = NodePath::from("hello");

    // GString vs others
    assert_eq!(s, sn);
    assert_eq!(s, np);
    assert_eq!(sn, s);
    assert_eq!(np, s);

    // StringName vs NodePath
    assert_eq!(sn, np);
    assert_eq!(np, sn);

    // Mismatches
    let s2 = GString::from("world");
    assert_ne!(s, s2);
    assert_ne!(s, StringName::from("world"));
    assert_ne!(s, NodePath::from("world"));
}

#[itest]
fn test_packed_array_accessors() {
    let mut array = PackedStringArray::new();
    array.push(&GString::from("first"));
    array.push(&GString::from("second"));

    // at_as / get_as
    let s1: String = array.at_as(0);
    assert_eq!(s1, "first");

    let s2: Option<String> = array.get_as(1);
    assert_eq!(s2, Some("second".to_string()));

    let s3: Option<String> = array.get_as(2);
    assert_eq!(s3, None);
}

#[itest]
fn test_variant_scalar_equality() {
    let v_int = Variant::from(42i64);
    assert_eq!(v_int, 42i32);
    assert_eq!(v_int, 42i16);
    assert_eq!(v_int, 42i8);
    assert_eq!(v_int, 42u32);
    assert_eq!(v_int, 42u16);
    assert_eq!(v_int, 42u8);

    let v_float = Variant::from(3.5f64);
    assert_eq!(v_float, 3.5f64);
}
