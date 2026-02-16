/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{StringName, Variant};
use crate::classes::Object;
use crate::meta::{arg_into_ref, AsArg, FromGodot, PropertyInfo, ToGodot};

/// Manual extensions for the `Object` class.
impl Object {
    /// ⚠️ Retrieves a property value, panicking if not found or cannot be converted to `T`.
    #[inline]
    #[track_caller]
    pub fn get_as<T: FromGodot>(&self, property: impl AsArg<StringName>) -> T {
        arg_into_ref!(property);
        let variant = self.get(property);
        if variant.is_nil() {
            panic!(
                "Object::get_as(): property '{property}' on {} not found (returned Nil)",
                self.get_class()
            );
        }
        variant.try_to::<T>().unwrap_or_else(|err| {
            panic!(
                "Object::get_as(): property '{property}' on {} conversion to {} failed: {err}",
                self.get_class(),
                std::any::type_name::<T>()
            );
        })
    }

    /// Retrieves a property value (fallible).
    #[inline]
    pub fn try_get_as<T: FromGodot>(&self, property: impl AsArg<StringName>) -> Option<T> {
        self.get(property).try_to::<T>().ok()
    }

    /// Sets a property value.
    #[inline]
    pub fn set_as<T: ToGodot>(&mut self, property: impl AsArg<StringName>, value: T) {
        self.set(property, &value.to_variant());
    }

    /// Returns `true` if the object has a property with the given `name`.
    #[inline]
    pub fn has_property(&self, name: impl AsArg<StringName>) -> bool {
        !self.get(name).is_nil()
    }

    /// ⚠️ Retrieves a metadata value, panicking if not found or cannot be converted to `T`.
    #[inline]
    #[track_caller]
    pub fn get_meta_as<T: FromGodot>(&self, name: impl AsArg<StringName>) -> T {
        arg_into_ref!(name);
        let variant = self.get_meta(name);
        if variant.is_nil() {
            panic!(
                "Object::get_meta_as(): meta '{name}' on {} not found (returned Nil)",
                self.get_class()
            );
        }
        variant.try_to::<T>().unwrap_or_else(|err| {
            panic!(
                "Object::get_meta_as(): meta '{name}' on {} conversion to {} failed: {err}",
                self.get_class(),
                std::any::type_name::<T>()
            );
        })
    }

    /// Retrieves a metadata value (fallible).
    #[inline]
    pub fn try_get_meta_as<T: FromGodot>(&self, name: impl AsArg<StringName>) -> Option<T> {
        self.get_meta(name).try_to::<T>().ok()
    }

    /// Sets a metadata value.
    #[inline]
    pub fn set_meta_as<T: ToGodot>(&mut self, name: impl AsArg<StringName>, value: T) {
        self.set_meta(name, &value.to_variant());
    }

    /// Returns `true` if the object has a metadata with the given `name`.
    #[inline]
    pub fn has_meta_alias(&self, name: impl AsArg<StringName>) -> bool {
        self.has_meta(name)
    }

    /// ⚠️ Calls a method and converts the return value to `T`, panicking if it fails.
    #[inline]
    #[track_caller]
    pub fn call_as<T: FromGodot>(&mut self, method: impl AsArg<StringName>, args: &[Variant]) -> T {
        arg_into_ref!(method);
        let result = self.call(method, args);
        result.try_to::<T>().unwrap_or_else(|err| {
            panic!(
                "Object::call_as(): method '{method}' on {} conversion to {} failed: {err}",
                self.get_class(),
                std::any::type_name::<T>()
            )
        })
    }

    /// Calls a method and converts the return value to `T` (fallible).
    #[inline]
    pub fn try_call_as<T: FromGodot>(
        &mut self,
        method: impl AsArg<StringName>,
        args: &[Variant],
    ) -> Option<T> {
        self.call(method, args).try_to::<T>().ok()
    }

    /// Returns the list of properties for this object as a `Vec<PropertyInfo>`.
    pub fn get_property_list_typed(&self) -> Vec<PropertyInfo> {
        self.get_property_list()
            .iter_shared()
            .map(|dict| PropertyInfo::from_dictionary(&dict))
            .collect()
    }
}
