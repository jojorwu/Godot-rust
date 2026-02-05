/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use crate::framework::itest;

#[itest]
fn test_any_array_join() {
    let arr = varray!["a", "b", "c"];
    let joined = arr.upcast_any_array().join(", ");
    assert_eq!(joined, GString::from("a, b, c"));

    let empty = VarArray::new();
    let joined_empty = empty.upcast_any_array().join("-");
    assert_eq!(joined_empty, GString::from(""));
}

#[itest]
fn test_packed_string_array_join() {
    let arr = PackedStringArray::from_iter(vec![GString::from("1"), GString::from("2"), GString::from("3")]);
    let joined = arr.join("|");
    assert_eq!(joined, GString::from("1|2|3"));
}
