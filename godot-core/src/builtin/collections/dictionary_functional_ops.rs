/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{Callable, VarDictionary};

/// Immutable, functional-programming operations for `Dictionary`, based on Godot callables.
///
/// Returned by [`VarDictionary::functional_ops()`].
pub struct DictionaryFunctionalOps<'a> {
    dict: &'a VarDictionary,
}

impl<'a> DictionaryFunctionalOps<'a> {
    pub(super) fn new(owner: &'a VarDictionary) -> Self {
        Self { dict: owner }
    }

    /// Returns a new dictionary containing only the elements for which the callable returns a truthy value.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    #[must_use]
    pub fn filter(&self, callable: &Callable) -> VarDictionary {
        let mut result = VarDictionary::new();
        for (key, value) in self.dict.iter_shared() {
            let args = [key.clone(), value.clone()];
            if callable.call(&args).booleanize() {
                result.set(key, value);
            }
        }
        result
    }

    /// Returns a new dictionary with each element transformed by the callable.
    ///
    /// The callable has signature `fn(key, value) -> Variant`.
    #[must_use]
    pub fn map(&self, callable: &Callable) -> VarDictionary {
        let mut result = VarDictionary::new();
        for (key, value) in self.dict.iter_shared() {
            let args = [key.clone(), value.clone()];
            let mapped = callable.call(&args);
            result.set(key, mapped);
        }
        result
    }
}
