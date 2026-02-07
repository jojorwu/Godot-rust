/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use crate::framework::itest;

#[itest]
fn test_dictionary_functional_ops() {
    let mut dict = VarDictionary::new();
    dict.set("a", 1i64);
    dict.set("b", 2i64);
    dict.set("c", 3i64);

    // filter
    let filter_callable = Callable::from_fn("filter_even", |args| {
        let _key = &args[0];
        let value = args[1].to::<i64>();
        value % 2 == 0
    });
    let filtered = dict.functional_ops().filter(&filter_callable);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered.at("b"), 2.to_variant());

    // map
    let map_callable = Callable::from_fn("map_square", |args| {
        let _key = &args[0];
        let value = args[1].to::<i64>();
        (value * value).to_variant()
    });
    let mapped = dict.functional_ops().map(&map_callable);
    assert_eq!(mapped.len(), 3);
    assert_eq!(mapped.at("a"), 1.to_variant());
    assert_eq!(mapped.at("b"), 4.to_variant());
    assert_eq!(mapped.at("c"), 9.to_variant());

    // any
    let any_even = Callable::from_fn("any_even", |args| {
        args[1].to::<i64>() % 2 == 0
    });
    assert!(dict.any(&any_even));
    assert!(dict.functional_ops().any(&any_even));

    // all
    let all_positive = Callable::from_fn("all_positive", |args| {
        args[1].to::<i64>() > 0
    });
    assert!(dict.all(&all_positive));
    assert!(dict.functional_ops().all(&all_positive));

    // reduce
    let reduce_sum = Callable::from_fn("reduce_sum", |args| {
        let acc = args[0].to::<i64>();
        let _key = &args[1];
        let value = args[2].to::<i64>();
        (acc + value).to_variant()
    });
    let sum = dict.functional_ops().reduce(&reduce_sum, &0.to_variant());
    assert_eq!(sum.to::<i64>(), 6);
}
