/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::builtin::{Array, GString, NodePath, PackedByteArray, StringName};
use godot::obj::{Gd, NewGd};

#[itest]
fn string_from_string_owned() {
    let s = String::from("ABC");
    let g = GString::from(s.clone());
    assert_eq!(g, "ABC");

    let sn = StringName::from(s.clone());
    assert_eq!(sn, "ABC");

    let np = NodePath::from(s);
    assert_eq!(np, "ABC");
}

#[itest]
fn string_into_iter() {
    let s = GString::from("ABC");
    let mut iter = s.into_iter();
    assert_eq!(iter.next(), Some('A'));
    assert_eq!(iter.next(), Some('B'));
    assert_eq!(iter.next(), Some('C'));
    assert_eq!(iter.next(), None);

    let s = GString::from("DEF");
    let mut count = 0;
    for (i, ch) in (&s).into_iter().enumerate() {
        match i {
            0 => assert_eq!(ch, 'D'),
            1 => assert_eq!(ch, 'E'),
            2 => assert_eq!(ch, 'F'),
            _ => panic!("too many characters"),
        }
        count += 1;
    }
    assert_eq!(count, 3);
}

#[cfg(since_api = "4.5")]
#[itest]
fn string_name_into_iter() {
    let s = StringName::from("ABC");
    let mut iter = s.into_iter();
    assert_eq!(iter.next(), Some('A'));
    assert_eq!(iter.next(), Some('B'));
    assert_eq!(iter.next(), Some('C'));
    assert_eq!(iter.next(), None);
}

#[itest]
fn string_equality_extensions() {
    let g = GString::from("hello");
    let s = String::from("hello");
    assert_eq!(g, s);
    assert_eq!(s, g);

    let _sn = StringName::from("world");
    assert_eq!(g, GString::from("hello")); // already works
    assert_eq!(g, "hello"); // already works

    let np = NodePath::from("path/to/node");
    assert_eq!(np, "path/to/node");
    assert_eq!("path/to/node", np);
    assert_eq!(np, String::from("path/to/node"));
}

#[itest]
fn packed_array_iter_shared() {
    let mut arr = PackedByteArray::new();
    arr.push(1);
    arr.push(2);

    let mut count = 0;
    for &val in arr.iter_shared() {
        count += val;
    }
    assert_eq!(count, 3);
}

#[itest]
fn packed_array_into_iter_owned() {
    let mut arr = PackedByteArray::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);

    let mut iter = arr.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[itest]
fn array_into_iter_owned() {
    let mut arr = Array::<i64>::new();
    arr.push(10);
    arr.push(20);

    let mut iter = arr.into_iter();
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), None);
}

#[itest]
fn array_at_as() {
    use godot::classes::RefCounted;

    let mut arr = Array::<Gd<RefCounted>>::new();
    let obj = RefCounted::new_gd();
    arr.push(&obj);

    let same_obj: Gd<RefCounted> = arr.at_as(0);
    assert_eq!(obj, same_obj);

    let maybe_obj: Option<Gd<RefCounted>> = arr.get_as(0);
    assert_eq!(Some(obj), maybe_obj);
}

#[itest]
fn test_static_node_path() {
    use godot::builtin::static_node_path;

    let path = static_node_path!("path/to/node");
    assert_eq!(path, &NodePath::from("path/to/node"));

    // Check caching
    let path2 = static_node_path!("path/to/node");
    assert!(std::ptr::eq(path, path2));
}
