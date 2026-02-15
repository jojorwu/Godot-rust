/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::builtin::{GString, NodePath, StringName, Variant};

#[itest]
fn string_equality() {
    let s = "Godot";
    let gs = GString::from(s);
    let sn = StringName::from(s);
    let np = NodePath::from(s);

    // GString
    assert!(gs == s);
    assert!(s == gs);

    // StringName
    assert!(sn == s);
    assert!(s == sn);

    // NodePath
    assert!(np == s);
    assert!(s == np);

    // Cross-comparisons
    assert!(gs == sn);
    assert!(sn == gs);
    assert!(gs == np);
    assert!(np == gs);
    assert!(sn == np);
    assert!(np == sn);

    // Non-equal
    assert!(gs != "Other");
    assert!(sn != "Other");
    assert!(np != "Other");
}

#[itest]
fn variant_equality_scalars() {
    let v_i64 = Variant::from(42i64);
    assert!(v_i64 == 42i64);
    assert!(42i64 == v_i64);
    assert!(v_i64 == 42i32);
    assert!(42i32 == v_i64);
    assert!(v_i64 == 42i16);
    assert!(v_i64 == 42i8);
    assert!(v_i64 == 42u32);
    assert!(v_i64 == 42u16);
    assert!(v_i64 == 42u8);

    let v_f64 = Variant::from(3.5f64);
    assert!(v_f64 == 3.5f64);
    assert!(3.5f64 == v_f64);
    assert!(v_f64 == 3.5f32);
    assert!(3.5f32 == v_f64);
}

#[itest]
fn variant_equality_strings() {
    let s = "Godot";
    let v = Variant::from(s);

    assert!(v == s);
    assert!(s == v);
}
