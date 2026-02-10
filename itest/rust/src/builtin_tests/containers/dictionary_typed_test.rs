/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::itest;
use godot::prelude::*;

#[itest]
fn test_dictionary_typed_iterators() {
    let mut dict = VarDictionary::new();
    dict.set("a", 1i64);
    dict.set("b", 2i64);

    // iter_typed
    let mut items: Vec<(GString, i64)> = dict.iter_typed::<GString, i64>().collect();
    items.sort_by_key(|(k, _)| k.to_string());
    assert_eq!(
        items,
        vec![(GString::from("a"), 1), (GString::from("b"), 2)]
    );

    // keys_typed
    let mut keys: Vec<GString> = dict.keys_typed::<GString>().collect();
    keys.sort_by_key(|k| k.to_string());
    assert_eq!(keys, vec![GString::from("a"), GString::from("b")]);

    // values_typed
    let mut values: Vec<i64> = dict.values_typed::<i64>().collect();
    values.sort();
    assert_eq!(values, vec![1, 2]);
}
