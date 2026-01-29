/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use crate::framework::itest;

#[itest]
fn any_array_find() {
    let typed = array![10, 20, 30, 20];
    let any = typed.upcast_any_array();

    assert_eq!(any.find(&20.to_variant(), None), Some(1));
    assert_eq!(any.find(&20.to_variant(), Some(2)), Some(3));
    assert_eq!(any.find(&40.to_variant(), None), None);
}

#[itest]
fn any_array_rfind() {
    let typed = array![10, 20, 30, 20];
    let any = typed.upcast_any_array();

    assert_eq!(any.rfind(&20.to_variant(), None), Some(3));
    assert_eq!(any.rfind(&20.to_variant(), Some(2)), Some(1));
    assert_eq!(any.rfind(&40.to_variant(), None), None);
}

#[itest]
fn any_array_typed_untyped_ops() {
    let typed = array![1, 2, 3];
    let mut any = typed.clone().upcast_any_array();

    assert_eq!(any.len(), 3);
    assert_eq!(any.at(1), 2.to_variant());

    any.reverse();
    assert_eq!(any.at(0), 3.to_variant());

    // Check if original was modified (since AnyArray is shallow copy of Array if we upcast)
    // Actually clone() was called before upcast_any_array in the line above.
    // typed is [1, 2, 3], any is [3, 2, 1] (but shared storage with the clone of typed)

    let mut shared_any = typed.clone().upcast_any_array();
    shared_any.reverse();
    assert_eq!(typed.at(0), 3);
}
