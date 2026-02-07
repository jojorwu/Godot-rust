/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::cell::OnceCell;
use std::marker::PhantomData;
use std::{fmt, ptr};

use godot_ffi as sys;
use sys::types::OpaqueDictionary;
use sys::{ffi_methods, interface_fn, GodotFfi};

use crate::builtin::{inner, Callable, StringName, VarArray, Variant, VariantType};
use crate::meta::{ElementType, ExtVariantType, FromGodot, ToGodot};

use super::dictionary_functional_ops::DictionaryFunctionalOps;

#[deprecated = "Renamed to `VarDictionary`; `Dictionary` will be reserved for typed dictionaries in the future."]
pub type Dictionary = VarDictionary;

/// Godot's `Dictionary` type.
///
/// Ordered associative hash-table, mapping keys to values.
///
/// The keys and values of the dictionary are all `Variant`s, so they can be of different types.
/// Variants are designed to be generally cheap to clone. Typed dictionaries are planned in a future godot-rust version.
///
/// Check out the [book](https://godot-rust.github.io/book/godot-api/builtins.html#arrays-and-dictionaries) for a tutorial on dictionaries.
///
/// # Dictionary example
///
/// ```no_run
/// # use godot::prelude::*;
/// // Create empty dictionary and add key-values pairs.
/// let mut dict = VarDictionary::new();
/// dict.set("str", "Hello");
/// dict.set("num", 23);
///
/// // Keys don't need to be strings.
/// let coord = Vector2i::new(0, 1);
/// dict.set(coord, "Tile77");
///
/// // Or create the same dictionary in a single expression.
/// let dict = vdict! {
///    "str": "Hello",
///    "num": 23,
///    coord: "Tile77",
/// };
///
/// // Access elements.
/// let value: Variant = dict.at("str");
/// let value: GString = dict.at("str").to(); // Variant::to() extracts GString.
/// let maybe: Option<Variant> = dict.get("absent_key");
///
/// // Iterate over key-value pairs as (Variant, Variant).
/// for (key, value) in dict.iter_shared() {
///     println!("{key} => {value}");
/// }
///
/// // Use typed::<K, V>() to get typed iterators.
/// for (key, value) in dict.iter_shared().typed::<GString, Variant>() {
///     println!("{key} => {value}");
/// }
///
/// // Clone dictionary (shares the reference), and overwrite elements through clone.
/// let mut cloned = dict.clone();
/// cloned.remove("num");
///
/// // Overwrite with set(); use insert() to get the previous value.
/// let prev = cloned.insert("str", "Goodbye"); // prev == Some("Hello")
///
/// // Changes will be reflected in the original dictionary.
/// assert_eq!(dict.at("str"), "Goodbye".to_variant());
/// assert_eq!(dict.get("num"), None);
/// ```
///
/// # Thread safety
///
/// The same principles apply as for [`VarArray`]. Consult its documentation for details.
///
/// # Godot docs
///
/// [`Dictionary` (stable)](https://docs.godotengine.org/en/stable/classes/class_dictionary.html)
pub struct VarDictionary {
    opaque: OpaqueDictionary,

    /// Lazily computed and cached element type information for the key type.
    cached_key_type: OnceCell<ElementType>,

    /// Lazily computed and cached element type information for the value type.
    cached_value_type: OnceCell<ElementType>,
}

impl VarDictionary {
    fn from_opaque(opaque: OpaqueDictionary) -> Self {
        Self {
            opaque,
            cached_key_type: OnceCell::new(),
            cached_value_type: OnceCell::new(),
        }
    }

    /// Constructs an empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// ⚠️ Returns the value for the given key, or panics.
    ///
    /// If you want to check for presence, use [`get()`][Self::get] or [`get_or_nil()`][Self::get_or_nil].
    ///
    /// # Panics
    ///
    /// If there is no value for the given key. Note that this is distinct from a `NIL` value, which is returned as `Variant::nil()`.
    #[inline]
    pub fn at<K: ToGodot>(&self, key: K) -> Variant {
        // Code duplication with get(), to avoid third clone (since K: ToGodot takes ownership).

        let key = key.to_variant();
        if self.contains_key(key.clone()) {
            self.get_or_nil(key)
        } else {
            panic!("key {key:?} missing in dictionary: {self:?}")
        }
    }

    /// Returns the value for the given key, or `None`.
    ///
    /// Note that `NIL` values are returned as `Some(Variant::nil())`, while absent values are returned as `None`.
    /// If you want to treat both as `NIL`, use [`get_or_nil()`][Self::get_or_nil].
    ///
    /// When you are certain that a key is present, use [`at()`][`Self::at`] instead.
    ///
    /// This can be combined with Rust's `Option` methods, e.g. `dict.get(key).unwrap_or(default)`.
    #[inline]
    pub fn get<K: ToGodot>(&self, key: K) -> Option<Variant> {
        // If implementation is changed, make sure to update at().

        let key = key.to_variant();
        if self.contains_key(key.clone()) {
            Some(self.get_or_nil(key))
        } else {
            None
        }
    }

    /// Returns the value for the given key, converted to `V`.
    ///
    /// # Panics
    /// If there is no value for the given key, or if the value cannot be converted to `V`.
    #[inline]
    pub fn at_as<K: ToGodot, V: FromGodot>(&self, key: K) -> V {
        self.at(key).to::<V>()
    }

    /// Returns the value for the given key, converted to `V`, or `None` if the key is absent or conversion fails.
    #[inline]
    pub fn get_as<K: ToGodot, V: FromGodot>(&self, key: K) -> Option<V> {
        self.get(key).and_then(|v| v.try_to::<V>().ok())
    }

    /// Returns the value at the key in the dictionary, or `NIL` otherwise.
    ///
    /// This method does not let you differentiate `NIL` values stored as values from absent keys.
    /// If you need that, use [`get()`][`Self::get`] instead.
    ///
    /// When you are certain that a key is present, use [`at()`][`Self::at`] instead.
    ///
    /// _Godot equivalent: `dict.get(key, null)`_
    pub fn get_or_nil<K: ToGodot>(&self, key: K) -> Variant {
        self.as_inner().get(&key.to_variant(), &Variant::nil())
    }

    /// Gets a value and ensures the key is set, inserting default if key is absent.
    ///
    /// If the `key` exists in the dictionary, this behaves like [`get()`][Self::get], and the existing value is returned.
    /// Otherwise, the `default` value is inserted and returned.
    ///
    /// # Compatibility
    /// This function is natively available from Godot 4.3 onwards, we provide a polyfill for older versions.
    ///
    /// _Godot equivalent: `get_or_add`_
    #[doc(alias = "get_or_add")]
    pub fn get_or_insert<K: ToGodot, V: ToGodot>(&mut self, key: K, default: V) -> Variant {
        self.balanced_ensure_mutable();

        let key_variant = key.to_variant();
        let default_variant = default.to_variant();

        // Godot 4.3+: delegate to native get_or_add().
        #[cfg(since_api = "4.3")]
        {
            self.as_inner().get_or_add(&key_variant, &default_variant)
        }

        // Polyfill for Godot versions before 4.3.
        #[cfg(before_api = "4.3")]
        {
            if let Some(existing_value) = self.get(key_variant.clone()) {
                existing_value
            } else {
                self.set(key_variant, default_variant.clone());
                default_variant
            }
        }
    }

    /// Returns `true` if the dictionary contains the given key.
    ///
    /// _Godot equivalent: `has`_
    #[doc(alias = "has")]
    pub fn contains_key<K: ToGodot>(&self, key: K) -> bool {
        let key = key.to_variant();
        self.as_inner().has(&key)
    }

    /// Returns `true` if the dictionary contains all the given keys.
    ///
    /// _Godot equivalent: `has_all`_
    #[doc(alias = "has_all")]
    pub fn contains_all_keys(&self, keys: &VarArray) -> bool {
        self.as_inner().has_all(keys)
    }

    /// Returns the number of entries in the dictionary.
    ///
    /// _Godot equivalent: `size`_
    #[doc(alias = "size")]
    #[inline]
    pub fn len(&self) -> usize {
        self.as_inner().size().try_into().unwrap()
    }

    /// Returns true if the dictionary is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_inner().is_empty()
    }

    /// Reverse-search a key by its value.
    ///
    /// Unlike Godot, this will return `None` if the key does not exist and `Some(Variant::nil())` the key is `NIL`.
    ///
    /// This operation is rarely needed and very inefficient. If you find yourself needing it a lot, consider
    /// using a `HashMap` or `Dictionary` with the inverse mapping (`V` -> `K`).
    ///
    /// _Godot equivalent: `find_key`_
    #[doc(alias = "find_key")]
    pub fn find_key_by_value<V: ToGodot>(&self, value: V) -> Option<Variant> {
        let key = self.as_inner().find_key(&value.to_variant());

        if !key.is_nil() || self.contains_key(key.clone()) {
            Some(key)
        } else {
            None
        }
    }

    /// Removes all key-value pairs from the dictionary.
    pub fn clear(&mut self) {
        self.balanced_ensure_mutable();

        self.as_inner().clear()
    }

    /// Set a key to a given value.
    ///
    /// If you are interested in the previous value, use [`insert()`][Self::insert] instead.
    ///
    /// _Godot equivalent: `dict[key] = value`_
    #[inline]
    pub fn set<K: ToGodot, V: ToGodot>(&mut self, key: K, value: V) {
        self.balanced_ensure_mutable();
        self.set_inner(key.to_variant(), value.to_variant());
    }

    /// Internal method to set a value without checking mutability.
    fn set_inner(&mut self, key: Variant, value: Variant) {
        // SAFETY: `self.get_ptr_mut(key)` always returns a valid pointer to a value in the dictionary; either pre-existing or newly inserted.
        unsafe {
            value.move_into_var_ptr(self.get_ptr_mut(key));
        }
    }

    /// Insert a value at the given key, returning the previous value for that key (if available).
    ///
    /// If you don't need the previous value, use [`set()`][Self::set] instead.
    #[must_use]
    #[inline]
    pub fn insert<K: ToGodot, V: ToGodot>(&mut self, key: K, value: V) -> Option<Variant> {
        self.balanced_ensure_mutable();

        let key = key.to_variant();
        let old_value = self.get(key.clone());
        self.set(key, value);
        old_value
    }

    /// Removes a key from the map, and returns the value associated with
    /// the key if the key was in the dictionary.
    ///
    /// _Godot equivalent: `erase`_
    #[doc(alias = "erase")]
    #[inline]
    pub fn remove<K: ToGodot>(&mut self, key: K) -> Option<Variant> {
        self.balanced_ensure_mutable();

        let key = key.to_variant();
        let old_value = self.get(key.clone());
        self.as_inner().erase(&key);
        old_value
    }

    crate::declare_hash_u32_method! {
        /// Returns a 32-bit integer hash value representing the dictionary and its contents.
    }

    /// Creates a new `Array` containing all the keys currently in the dictionary.
    ///
    /// _Godot equivalent: `keys`_
    #[doc(alias = "keys")]
    pub fn keys_array(&self) -> VarArray {
        // SAFETY: keys() returns an untyped array with element type Variant.
        let out_array = self.as_inner().keys();
        unsafe { out_array.assume_type() }
    }

    /// Returns a `Vec` containing all the keys in the dictionary.
    pub fn keys(&self) -> Vec<Variant> {
        Vec::from(&self.keys_array())
    }

    /// Returns a `Vec` containing all the keys in the dictionary, converted to `K`.
    ///
    /// # Panics
    /// If any key cannot be converted to `K`.
    pub fn typed_keys<K: FromGodot>(&self) -> Vec<K> {
        self.keys_array().iter_shared().map(|k| k.to::<K>()).collect()
    }

    /// Creates a new `Array` containing all the values currently in the dictionary.
    ///
    /// _Godot equivalent: `values`_
    #[doc(alias = "values")]
    pub fn values_array(&self) -> VarArray {
        // SAFETY: values() returns an untyped array with element type Variant.
        let out_array = self.as_inner().values();
        unsafe { out_array.assume_type() }
    }

    /// Returns a `Vec` containing all the values in the dictionary.
    pub fn values(&self) -> Vec<Variant> {
        Vec::from(&self.values_array())
    }

    /// Returns a `Vec` containing all the values in the dictionary, converted to `V`.
    ///
    /// # Panics
    /// If any value cannot be converted to `V`.
    pub fn typed_values<V: FromGodot>(&self) -> Vec<V> {
        self.values_array().iter_shared().map(|v| v.to::<V>()).collect()
    }

    /// Copies all keys and values from `other` into `self`.
    ///
    /// If `overwrite` is true, it will overwrite pre-existing keys.
    ///
    /// _Godot equivalent: `merge`_
    #[doc(alias = "merge")]
    pub fn extend_dictionary(&mut self, other: &Self, overwrite: bool) {
        self.balanced_ensure_mutable();

        self.as_inner().merge(other, overwrite)
    }

    /// Deep copy, duplicating nested collections.
    ///
    /// All nested arrays and dictionaries are duplicated and will not be shared with the original dictionary.
    /// Note that any `Object`-derived elements will still be shallow copied.
    ///
    /// To create a shallow copy, use [`Self::duplicate_shallow()`] instead.  
    /// To create a new reference to the same dictionary data, use [`clone()`][Clone::clone].
    ///
    /// _Godot equivalent: `dict.duplicate(true)`_
    pub fn duplicate_deep(&self) -> Self {
        self.as_inner().duplicate(true).with_cache(self)
    }

    /// Shallow copy, copying elements but sharing nested collections.
    ///
    /// All dictionary keys and values are copied, but any reference types (such as `Array`, `Dictionary` and `Gd<T>` objects)
    /// will still refer to the same value.
    ///
    /// To create a deep copy, use [`Self::duplicate_deep()`] instead.
    /// To create a new reference to the same dictionary data, use [`clone()`][Clone::clone].
    ///
    /// _Godot equivalent: `dict.duplicate(false)`_
    pub fn duplicate_shallow(&self) -> Self {
        self.as_inner().duplicate(false).with_cache(self)
    }

    /// Returns an iterator over the key-value pairs of the `Dictionary`.
    ///
    /// The pairs are each of type `(Variant, Variant)`. Each pair references the original dictionary, but instead of a `&`-reference
    /// to key-value pairs as you might expect, the iterator returns a (cheap, shallow) copy of each key-value pair.
    ///
    /// Note that it's possible to modify the dictionary through another reference while iterating over it. This will not result in
    /// unsoundness or crashes, but will cause the iterator to behave in an unspecified way.
    ///
    /// Use `dict.iter_shared().typed::<K, V>()` to iterate over `(K, V)` pairs instead.
    pub fn iter_shared(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Returns an iterator over the keys in a `Dictionary`.
    ///
    /// The keys are each of type `Variant`. Each key references the original `Dictionary`, but instead of a `&`-reference to keys pairs
    /// as you might expect, the iterator returns a (cheap, shallow) copy of each key pair.
    ///
    /// Note that it's possible to modify the `Dictionary` through another reference while iterating over it. This will not result in
    /// unsoundness or crashes, but will cause the iterator to behave in an unspecified way.
    ///
    /// Use `dict.keys_shared().typed::<K>()` to iterate over `K` keys instead.
    pub fn keys_shared(&self) -> Keys<'_> {
        Keys::new(self)
    }

    /// Returns an iterator over the values in a `Dictionary`.
    ///
    /// The values are each of type `Variant`. Each value references the original `Dictionary`, but instead of a `&`-reference to values pairs
    /// as you might expect, the iterator returns a (cheap, shallow) copy of each value pair.
    ///
    /// Note that it's possible to modify the `Dictionary` through another reference while iterating over it. This will not result in
    /// unsoundness or crashes, but will cause the iterator to behave in an unspecified way.
    ///
    /// Use `dict.values_shared().typed::<V>()` to iterate over `V` values instead.
    pub fn values_shared(&self) -> Values<'_> {
        Values::new(self)
    }

    /// Returns a typed iterator over key-value pairs.
    pub fn iter_typed<K: FromGodot, V: FromGodot>(&self) -> TypedIter<'_, K, V> {
        self.iter_shared().typed::<K, V>()
    }

    /// Returns a typed iterator over keys.
    pub fn keys_typed<K: FromGodot>(&self) -> TypedKeys<'_, K> {
        self.keys_shared().typed::<K>()
    }

    /// Returns a typed iterator over values.
    pub fn values_typed<V: FromGodot>(&self) -> TypedValues<'_, V> {
        TypedValues::new(self)
    }

    /// Access to Godot's functional-programming APIs based on callables.
    pub fn functional_ops(&self) -> DictionaryFunctionalOps<'_> {
        DictionaryFunctionalOps::new(self)
    }

    /// Returns a new dictionary containing only the elements for which the callable returns a truthy value.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    #[must_use]
    pub fn filter(&self, callable: &Callable) -> VarDictionary {
        self.functional_ops().filter(callable)
    }

    /// Returns a new dictionary with each element transformed by the callable.
    ///
    /// The callable has signature `fn(key, value) -> Variant`.
    #[must_use]
    pub fn map(&self, callable: &Callable) -> VarDictionary {
        self.functional_ops().map(callable)
    }

    /// Returns `true` if the callable returns a truthy value for at least one element.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    pub fn any(&self, callable: &Callable) -> bool {
        self.functional_ops().any(callable)
    }

    /// Returns `true` if the callable returns a truthy value for all elements.
    ///
    /// The callable has signature `fn(key, value) -> bool`.
    pub fn all(&self, callable: &Callable) -> bool {
        self.functional_ops().all(callable)
    }

    /// Turns the dictionary into a shallow-immutable dictionary.
    ///
    /// Makes the dictionary read-only and returns the original dictionary. Disables modification of the dictionary's contents.
    /// Does not apply to nested content, e.g. elements of nested dictionaries.
    ///
    /// In GDScript, dictionaries are automatically read-only if declared with the `const` keyword.
    ///
    /// # Semantics and alternatives
    /// You can use this in Rust, but the behavior of mutating methods is only validated in a best-effort manner (more than in GDScript though):
    /// some methods like `set()` panic in Debug mode, when used on a read-only dictionary. There is no guarantee that any attempts to change
    /// result in feedback; some may silently do nothing.
    ///
    /// In Rust, you can use shared references (`&Dictionary`) to prevent mutation. Note however that `Clone` can be used to create another
    /// reference, through which mutation can still occur. For deep-immutable dictionaries, you'll need to keep your `Dictionary` encapsulated
    /// or directly use Rust data structures.
    ///
    /// _Godot equivalent: `make_read_only`_
    #[doc(alias = "make_read_only")]
    pub fn into_read_only(self) -> Self {
        self.as_inner().make_read_only();
        self
    }

    /// Returns true if the dictionary is read-only.
    ///
    /// See [`into_read_only()`][Self::into_read_only].
    /// In GDScript, dictionaries are automatically read-only if declared with the `const` keyword.
    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.as_inner().is_read_only()
    }

    /// Best-effort mutability check.
    ///
    /// # Panics (safeguards-balanced)
    /// If the dictionary is marked as read-only.
    fn balanced_ensure_mutable(&self) {
        sys::balanced_assert!(
            !self.is_read_only(),
            "mutating operation on read-only dictionary"
        );
    }

    /// Returns the runtime element type information for keys in this dictionary.
    ///
    /// Provides information about Godot typed dictionaries, even though godot-rust currently doesn't implement generics for those.
    ///
    /// The result is generally cached, so feel free to call this method repeatedly.
    ///
    /// # Panics (Debug)
    /// In the astronomically rare case where another extension in Godot modifies a dictionary's key type (which godot-rust already cached as `Untyped`)
    /// via C function `dictionary_set_typed`, thus leading to incorrect cache values. Such bad practice of not typing dictionaries immediately on
    /// construction is not supported, and will not be checked in Release mode.
    #[cfg(since_api = "4.4")]
    pub fn key_element_type(&self) -> ElementType {
        ElementType::get_or_compute_cached(
            &self.cached_key_type,
            || self.as_inner().get_typed_key_builtin(),
            || self.as_inner().get_typed_key_class_name(),
            || self.as_inner().get_typed_key_script(),
        )
    }

    /// Returns the runtime element type information for values in this dictionary.
    ///
    /// Provides information about Godot typed dictionaries, even though godot-rust currently doesn't implement generics for those.
    ///
    /// The result is generally cached, so feel free to call this method repeatedly.
    ///
    /// # Panics (Debug)
    /// In the astronomically rare case where another extension in Godot modifies a dictionary's value type (which godot-rust already cached as `Untyped`)
    /// via C function `dictionary_set_typed`, thus leading to incorrect cache values. Such bad practice of not typing dictionaries immediately on
    /// construction is not supported, and will not be checked in Release mode.
    #[cfg(since_api = "4.4")]
    pub fn value_element_type(&self) -> ElementType {
        ElementType::get_or_compute_cached(
            &self.cached_value_type,
            || self.as_inner().get_typed_value_builtin(),
            || self.as_inner().get_typed_value_class_name(),
            || self.as_inner().get_typed_value_script(),
        )
    }

    /// Reserves capacity for at least `capacity` elements.
    ///
    /// The dictionary may reserve more space to avoid frequent reallocations.
    ///
    /// _Godot equivalent: `reserve`_
    #[cfg(since_api = "4.3")]
    pub fn reserve(&mut self, capacity: usize) {
        self.balanced_ensure_mutable();

        let variant = self.to_variant();
        let method = crate::static_sname!(c"reserve");
        let arg = Variant::from(capacity as i64);
        let _result_variant = variant.call(method, &[arg]);

        // Variant::call() on a Dictionary modifies it in-place.
        // We re-assign from the variant to ensure COW changes are picked up.
        // If the call failed, the variant might return Nil.
        if variant.get_type() == VariantType::DICTIONARY {
            *self = variant.to::<Self>();
        }
    }

    #[doc(hidden)]
    pub fn as_inner(&self) -> inner::InnerDictionary<'_> {
        inner::InnerDictionary::from_outer(self)
    }

    /// Get the pointer corresponding to the given key in the dictionary.
    ///
    /// If there exists no value at the given key, a `NIL` variant will be inserted for that key.
    fn get_ptr_mut<K: ToGodot>(&mut self, key: K) -> sys::GDExtensionVariantPtr {
        let key = key.to_variant();

        // Never a null pointer, since entry either existed already or was inserted above.
        // SAFETY: accessing an unknown key _mutably_ creates that entry in the dictionary, with value `NIL`.
        unsafe { interface_fn!(dictionary_operator_index)(self.sys_mut(), key.var_sys()) }
    }

    /// Execute a function that creates a new dictionary, transferring cached element types if available.
    ///
    /// This is a convenience helper for methods that create new dictionary instances and want to preserve
    /// cached type information to avoid redundant FFI calls.
    fn with_cache(self, source: &Self) -> Self {
        // Transfer both key and value type caches independently
        ElementType::transfer_cache(&source.cached_key_type, &self.cached_key_type);
        ElementType::transfer_cache(&source.cached_value_type, &self.cached_value_type);
        self
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Traits

// SAFETY:
// - `move_return_ptr`
//   Nothing special needs to be done beyond a `std::mem::swap` when returning a dictionary.
//   So we can just use `ffi_methods`.
//
// - `from_arg_ptr`
//   Dictionaries are properly initialized through a `from_sys` call, but the ref-count should be
//   incremented as that is the callee's responsibility. Which we do by calling
//   `std::mem::forget(dictionary.clone())`.
unsafe impl GodotFfi for VarDictionary {
    const VARIANT_TYPE: ExtVariantType = ExtVariantType::Concrete(sys::VariantType::DICTIONARY);

    ffi_methods! { type sys::GDExtensionTypePtr = *mut Opaque; .. }
}

crate::meta::impl_godot_as_self!(VarDictionary: ByRef);

impl_builtin_traits! {
    for VarDictionary {
        Default => dictionary_construct_default;
        Drop => dictionary_destroy;
        PartialEq => dictionary_operator_equal;
        // No < operator for dictionaries.
        // Hash could be added, but without Eq it's not that useful.
    }
}

impl fmt::Debug for VarDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_variant().stringify())
    }
}

impl fmt::Display for VarDictionary {
    /// Formats `Dictionary` to match Godot's string representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (count, (key, value)) in self.iter_shared().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key}: {value}")?;
        }
        write!(f, " }}")
    }
}

/// Creates a new reference to the data in this dictionary. Changes to the original dictionary will be
/// reflected in the copy and vice versa.
///
/// To create a (mostly) independent copy instead, see [`VarDictionary::duplicate_shallow()`] and
/// [`VarDictionary::duplicate_deep()`].
impl Clone for VarDictionary {
    fn clone(&self) -> Self {
        // SAFETY: `self` is a valid dictionary, since we have a reference that keeps it alive.
        let result = unsafe {
            Self::new_with_uninit(|self_ptr| {
                let ctor = sys::builtin_fn!(dictionary_construct_copy);
                let args = [self.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        };
        result.with_cache(self)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Conversion traits

/// Creates a dictionary from the given iterator `I` over a `(&K, &V)` key-value pair.
///
/// Each key and value are converted to a `Variant`.
impl<'a, 'b, K, V, I> From<I> for VarDictionary
where
    I: IntoIterator<Item = (&'a K, &'b V)>,
    K: ToGodot + 'a,
    V: ToGodot + 'b,
{
    fn from(iterable: I) -> Self {
        iterable
            .into_iter()
            .map(|(key, value)| (key.to_variant(), value.to_variant()))
            .collect()
    }
}

/// Insert iterator range into dictionary.
///
/// Inserts all key-value pairs from the iterator into the dictionary. Previous values for keys appearing
/// in `iter` will be overwritten.
impl<K: ToGodot, V: ToGodot> Extend<(K, V)> for VarDictionary {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        self.balanced_ensure_mutable();
        for (k, v) in iter.into_iter() {
            self.set_inner(k.to_variant(), v.to_variant())
        }
    }
}

impl<K: ToGodot, V: ToGodot> FromIterator<(K, V)> for VarDictionary {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut dict = VarDictionary::new();
        dict.extend(iter);
        dict
    }
}

impl IntoIterator for VarDictionary {
    type Item = (Variant, Variant);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a VarDictionary {
    type Item = (Variant, Variant);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_shared()
    }
}

/// An iterator that consumes a [`VarDictionary`] and yields its key-value pairs.
pub struct IntoIter {
    last_key: Option<Variant>,
    dictionary: VarDictionary,
    variant_dict: Variant,
    is_first: bool,
    next_idx: usize,
}

impl IntoIter {
    fn new(dictionary: VarDictionary) -> Self {
        let variant_dict = dictionary.to_variant();
        Self {
            last_key: None,
            dictionary,
            variant_dict,
            is_first: true,
            next_idx: 0,
        }
    }

    fn next_key(&mut self) -> Option<Variant> {
        let new_key = if self.is_first {
            self.is_first = false;
            DictionaryIter::call_init_internal(&self.variant_dict)
        } else {
            DictionaryIter::call_next_internal(&self.variant_dict, self.last_key.take()?)
        };

        if self.next_idx < self.dictionary.len() {
            self.next_idx += 1;
        }

        self.last_key.clone_from(&new_key);
        new_key
    }
}

impl Iterator for IntoIter {
    type Item = (Variant, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.next_key()?;

        // Use as_inner().get() directly to avoid recursive into_iter() issues if we had them.
        let value = self.dictionary.as_inner().get(&key, &Variant::nil());
        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = usize::saturating_sub(self.dictionary.len(), self.next_idx);
        (remaining, Some(remaining))
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Internal helper for different iterator impls -- not an iterator itself
struct DictionaryIter<'a> {
    last_key: Option<Variant>,
    dictionary: &'a VarDictionary,
    variant_dict: Variant,
    is_first: bool,
    next_idx: usize,
}

impl<'a> DictionaryIter<'a> {
    fn new(dictionary: &'a VarDictionary) -> Self {
        let variant_dict = dictionary.to_variant();
        Self {
            last_key: None,
            dictionary,
            variant_dict,
            is_first: true,
            next_idx: 0,
        }
    }

    fn next_key(&mut self) -> Option<Variant> {
        let new_key = if self.is_first {
            self.is_first = false;
            Self::call_init_internal(&self.variant_dict)
        } else {
            Self::call_next_internal(&self.variant_dict, self.last_key.take()?)
        };

        if self.next_idx < self.dictionary.len() {
            self.next_idx += 1;
        }

        self.last_key.clone_from(&new_key);
        new_key
    }

    fn next_key_value(&mut self) -> Option<(Variant, Variant)> {
        let key = self.next_key()?;
        if !self.dictionary.contains_key(key.clone()) {
            return None;
        }

        let value = self.dictionary.as_inner().get(&key, &Variant::nil());
        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Need to check for underflow in case any entry was removed while
        // iterating (i.e. next_index > dicitonary.len())
        let remaining = usize::saturating_sub(self.dictionary.len(), self.next_idx);

        (remaining, Some(remaining))
    }

    fn call_init_internal(variant_dict: &Variant) -> Option<Variant> {
        let variant: Variant = Variant::nil();
        let iter_fn = |dictionary, next_value: sys::GDExtensionVariantPtr, valid| unsafe {
            interface_fn!(variant_iter_init)(dictionary, sys::SysPtr::as_uninit(next_value), valid)
        };

        Self::ffi_iterate(iter_fn, variant_dict, variant)
    }

    fn call_next_internal(variant_dict: &Variant, last_key: Variant) -> Option<Variant> {
        let iter_fn = |dictionary, next_value, valid| unsafe {
            interface_fn!(variant_iter_next)(dictionary, next_value, valid)
        };

        Self::ffi_iterate(iter_fn, variant_dict, last_key)
    }

    /// Calls the provided Godot FFI function, in order to iterate the current state.
    ///
    /// # Safety:
    /// `iter_fn` must point to a valid function that interprets the parameters according to their type specification.
    fn ffi_iterate(
        iter_fn: unsafe fn(
            sys::GDExtensionConstVariantPtr,
            sys::GDExtensionVariantPtr,
            *mut sys::GDExtensionBool,
        ) -> sys::GDExtensionBool,
        variant_dict: &Variant,
        mut next_value: Variant,
    ) -> Option<Variant> {
        let mut valid_u8: u8 = 0;

        // SAFETY:
        // `dictionary` is a valid dictionary since we have a reference to it,
        //    so this will call the implementation for dictionaries.
        // `last_key` is an initialized and valid `Variant`, since we own a copy of it.
        let has_next = unsafe {
            iter_fn(
                variant_dict.var_sys(),
                next_value.var_sys_mut(),
                ptr::addr_of_mut!(valid_u8),
            )
        };
        let valid = u8_to_bool(valid_u8);
        let has_next = u8_to_bool(has_next);

        if has_next {
            assert!(valid);
            Some(next_value)
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Iterator over key-value pairs in a [`VarDictionary`].
///
/// See [`VarDictionary::iter_shared()`] for more information about iteration over dictionaries.
pub struct Iter<'a> {
    iter: DictionaryIter<'a>,
}

impl<'a> Iter<'a> {
    fn new(dictionary: &'a VarDictionary) -> Self {
        Self {
            iter: DictionaryIter::new(dictionary),
        }
    }

    /// Creates an iterator that converts each `(Variant, Variant)` key-value pair into a `(K, V)` key-value
    /// pair, panicking upon conversion failure.
    pub fn typed<K: FromGodot, V: FromGodot>(self) -> TypedIter<'a, K, V> {
        TypedIter::from_untyped(self)
    }
}

impl Iterator for Iter<'_> {
    type Item = (Variant, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_key_value()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// [`VarDictionary`] iterator that converts each value into a typed `V`.
///
/// See [`VarDictionary::iter_shared()`] for more information about iteration over dictionaries.
pub struct TypedValues<'a, V> {
    iter: DictionaryIter<'a>,
    _v: PhantomData<V>,
}

impl<'a, V> TypedValues<'a, V> {
    fn new(dictionary: &'a VarDictionary) -> Self {
        Self {
            iter: DictionaryIter::new(dictionary),
            _v: PhantomData,
        }
    }

    /// Creates a typed iterator from an untyped one.
    pub fn from_untyped(value: Values<'a>) -> Self {
        Self {
            iter: value.iter,
            _v: PhantomData,
        }
    }
}

impl<V: FromGodot> Iterator for TypedValues<'_, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_key_value().map(|(_k, v)| V::from_variant(&v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Iterator over values in a [`VarDictionary`].
///
/// See [`VarDictionary::values_shared()`] for more information about iteration over dictionaries.
pub struct Values<'a> {
    iter: DictionaryIter<'a>,
}

impl<'a> Values<'a> {
    fn new(dictionary: &'a VarDictionary) -> Self {
        Self {
            iter: DictionaryIter::new(dictionary),
        }
    }

    /// Creates an iterator that will convert each `Variant` value into a value of type `V`,
    /// panicking upon failure to convert.
    pub fn typed<V: FromGodot>(self) -> TypedValues<'a, V> {
        TypedValues::from_untyped(self)
    }

    /// Returns an array of the values.
    pub fn array(self) -> VarArray {
        assert!(self.iter.is_first);
        self.iter.dictionary.values_array()
    }
}

impl Iterator for Values<'_> {
    type Item = Variant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_key_value().map(|(_k, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Iterator over keys in a [`VarDictionary`].
///
/// See [`VarDictionary::keys_shared()`] for more information about iteration over dictionaries.
pub struct Keys<'a> {
    iter: DictionaryIter<'a>,
}

impl<'a> Keys<'a> {
    fn new(dictionary: &'a VarDictionary) -> Self {
        Self {
            iter: DictionaryIter::new(dictionary),
        }
    }

    /// Creates an iterator that will convert each `Variant` key into a key of type `K`,
    /// panicking upon failure to convert.
    pub fn typed<K: FromGodot>(self) -> TypedKeys<'a, K> {
        TypedKeys::from_untyped(self)
    }

    /// Returns an array of the keys.
    pub fn array(self) -> VarArray {
        // Can only be called
        assert!(self.iter.is_first);
        self.iter.dictionary.keys_array()
    }
}

impl Iterator for Keys<'_> {
    type Item = Variant;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_key()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// [`VarDictionary`] iterator that converts each key-value pair into a typed `(K, V)`.
///
/// See [`VarDictionary::iter_shared()`] for more information about iteration over dictionaries.
pub struct TypedIter<'a, K, V> {
    iter: DictionaryIter<'a>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<'a, K, V> TypedIter<'a, K, V> {
    fn from_untyped(value: Iter<'a>) -> Self {
        Self {
            iter: value.iter,
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

impl<K: FromGodot, V: FromGodot> Iterator for TypedIter<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next_key_value()
            .map(|(key, value)| (K::from_variant(&key), V::from_variant(&value)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// [`VarDictionary`] iterator that converts each key into a typed `K`.
///
/// See [`VarDictionary::iter_shared()`] for more information about iteration over dictionaries.
pub struct TypedKeys<'a, K> {
    iter: DictionaryIter<'a>,
    _k: PhantomData<K>,
}

impl<'a, K> TypedKeys<'a, K> {
    fn from_untyped(value: Keys<'a>) -> Self {
        Self {
            iter: value.iter,
            _k: PhantomData,
        }
    }
}

impl<K: FromGodot> Iterator for TypedKeys<'_, K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_key().map(|k| K::from_variant(&k))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Helper functions

fn u8_to_bool(u: u8) -> bool {
    match u {
        0 => false,
        1 => true,
        _ => panic!("Invalid boolean value {u}"),
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Constructs [`VarDictionary`] literals, close to Godot's own syntax.
///
/// Any value can be used as a key, but to use an expression you need to surround it
/// in `()` or `{}`.
///
/// # Example
/// ```no_run
/// use godot::builtin::{vdict, Variant};
///
/// let key = "my_key";
/// let d = vdict! {
///     "key1": 10,
///     "another": Variant::nil(),
///     key: true,
///     (1 + 2): "final",
/// };
/// ```
///
/// # See also
///
/// For arrays, similar macros [`array!`][macro@crate::builtin::array] and [`varray!`][macro@crate::builtin::varray] exist.
#[macro_export]
macro_rules! vdict {
    ($($key:tt: $value:expr),* $(,)?) => {
        {
            let mut d = $crate::builtin::VarDictionary::new();
            $(
                // `cargo check` complains that `(1 + 2): true` has unused parens, even though it's not
                // possible to omit the parens.
                #[allow(unused_parens)]
                d.set($key, $value);
            )*
            d
        }
    };
}

#[macro_export]
#[deprecated = "Migrate to `vdict!`. The name `dict!` will be used in the future for typed dictionaries."]
macro_rules! dict {
    ($($key:tt: $value:expr),* $(,)?) => {
        $crate::vdict!(
            $($key: $value),*
        )
    };
}
