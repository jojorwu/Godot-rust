/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{Callable, VarDictionary, Variant};

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

    /// Reduces the dictionary to a single value by iteratively applying the callable.
    ///
    /// The callable takes three arguments: the accumulator, the current key and the current value.
    /// It returns the new accumulator value. The process starts with `initial` as the accumulator.
    #[must_use]
    pub fn reduce(&self, callable: &Callable, initial: &Variant) -> Variant {
        let mut acc = initial.clone();
        for (key, value) in self.dict.iter_shared() {
            let args = [acc, key, value];
            acc = callable.call(&args);
        }
        acc
    }

    /// Returns `true` if the callable returns a truthy value for at least one element.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    pub fn any(&self, callable: &Callable) -> bool {
        for (key, value) in self.dict.iter_shared() {
            let args = [key, value];
            if callable.call(&args).booleanize() {
                return true;
            }
        }
        false
    }

    /// Returns `true` if the callable returns a truthy value for all elements.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    pub fn all(&self, callable: &Callable) -> bool {
        for (key, value) in self.dict.iter_shared() {
            let args = [key, value];
            if !callable.call(&args).booleanize() {
                return false;
            }
        }
        true
    }
}
