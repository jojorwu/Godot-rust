/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use godot::meta::VariantBorrow;
use crate::framework::itest;

#[itest]
fn variant_borrow_variant() {
    let v = Variant::from(42);
    let borrowed = <Variant as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, &v);
    assert_eq!(borrowed.to::<i64>(), 42);
}

#[itest]
fn variant_borrow_string() {
    let s = GString::from("hello");
    let v = Variant::from(s.clone());
    let borrowed = <GString as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, s);
}

#[itest]
fn variant_borrow_array() {
    let arr = array![1, 2, 3];
    let v = Variant::from(arr.clone());
    let borrowed = <Array<i32> as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, arr);
}

#[itest]
fn variant_borrow_geometric() {
    let b = Basis::IDENTITY;
    let v = Variant::from(b);
    let borrowed = <Basis as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, b);

    let t = Transform3D::IDENTITY;
    let v = Variant::from(t);
    let borrowed = <Transform3D as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, t);

    let a = Aabb::new(Vector3::ZERO, Vector3::ONE);
    let v = Variant::from(a);
    let borrowed = <Aabb as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, a);

    let c = Color::from_rgba(0.1, 0.2, 0.3, 0.4);
    let v = Variant::from(c);
    let borrowed = <Color as VariantBorrow>::borrow_from_variant(&v);
    assert_eq!(borrowed, c);
}
