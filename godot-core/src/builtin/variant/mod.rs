/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fmt, ptr};

use godot_ffi as sys;
use sys::{ffi_methods, interface_fn, GodotFfi};

use crate::builtin::{
    GString, NodePath, StringName, VarArray, VariantDispatch, VariantOperator, VariantType,
};
use crate::classes;
use crate::meta::error::{ConvertError, FromVariantError};
use crate::meta::{
    arg_into_ref, ffi_variant_type, ArrayElement, AsArg, EngineFromGodot, ExtVariantType,
    FromGodot, GodotType, ToGodot,
};

mod impls;

/// Godot variant type, able to store a variety of different types.
///
/// While Godot variants do not appear very frequently in Rust due to their lack of compile-time type-safety, they are central to all sorts of
/// dynamic APIs. For example, if you want to call a method on an object based on a string, you will need variants to store arguments and return
/// value.  
///
/// # Conversions
///
/// For type conversions, please read the [`godot::meta` module docs][crate::meta].
///
/// # Godot docs
///
/// [`Variant` (stable)](https://docs.godotengine.org/en/stable/classes/class_variant.html)
// We rely on the layout of `Variant` being the same as Godot's layout in `borrow_slice` and `borrow_slice_mut`.
#[repr(transparent)]
pub struct Variant {
    _opaque: sys::types::OpaqueVariant,
}

impl Variant {
    /// Create an empty variant (`null` value in GDScript).
    ///
    /// If a Godot engine API accepts object (not variant) parameters and you'd like to pass `null`, use
    /// [`Gd::null_arg()`][crate::obj::Gd::null_arg] instead.
    #[inline]
    pub fn nil() -> Self {
        Self::default()
    }

    /// Create a variant holding a non-nil value.
    ///
    /// Equivalent to [`value.to_variant()`][ToGodot::to_variant], but consumes the argument.
    #[inline]
    pub fn from<T: ToGodot>(value: T) -> Self {
        value.to_variant()
    }

    /// ⚠️ Convert to type `T`, panicking on failure.
    ///
    /// Equivalent to [`T::from_variant(&self)`][FromGodot::from_variant].
    ///
    /// # Panics
    /// When this variant holds a different type.
    pub fn to<T: FromGodot>(&self) -> T {
        T::from_variant(self)
    }

    /// Convert to type `T`, returning `Err` on failure.
    ///
    /// The conversion only succeeds if the type stored in the variant matches `T`'s FFI representation.
    /// For lenient conversions like in GDScript, use [`try_to_relaxed()`](Self::try_to_relaxed) instead.
    ///
    /// Equivalent to [`T::try_from_variant(&self)`][FromGodot::try_from_variant].
    pub fn try_to<T: FromGodot>(&self) -> Result<T, ConvertError> {
        T::try_from_variant(self)
    }

    /// Convert to `T` using Godot's less strict conversion rules.
    ///
    /// More lenient than [`try_to()`](Self::try_to), which only allows exact type matches.
    /// Enables conversions between related types that Godot considers compatible under its conversion rules.
    ///
    /// Precisely matches GDScript's behavior to converts arguments, when a function declares a parameter of different type.
    ///
    /// # Conversion diagram
    /// Exhaustive list of all possible conversions, as of Godot 4.6. The arrow `──►` means "converts to".
    ///
    /// ```text
    ///                                                               * ───► Variant
    ///                                                               * ───► itself (reflexive)
    ///         float          StringName
    ///         ▲   ▲             ▲                            Vector2 ◄───► Vector2i
    ///        ╱     ╲            │                            Vector3 ◄───► Vector3i
    ///       ▼       ▼           ▼                            Vector4 ◄───► Vector4i
    ///    bool ◄───► int       GString ◄───► NodePath           Rect2 ◄───► Rect2i
    ///                 ╲       ╱
    ///                  ╲     ╱                              Array<T> ◄───► PackedArray<T>
    ///                   ▼   ▼
    ///                   Color                                   Gd<T> ───► Rid
    ///                                                             nil ───► Option<Gd<T>>
    ///
    ///                                Basis ◄───► Quaternion
    ///                                    ╲       ╱
    ///                                     ╲     ╱
    ///                                      ▼   ▼
    ///                 Transform2D ◄───► Transform3D ◄───► Projection
    /// ```
    ///
    /// # Godot implementation details
    /// See [GDExtension interface](https://github.com/godotengine/godot/blob/4.6-stable/core/extension/gdextension_interface.h#L1353-L1364)
    /// and [C++ implementation](https://github.com/godotengine/godot/blob/4.6-stable/core/variant/variant.cpp#L532) (Godot 4.6 at the time of
    /// writing). The "strict" part refers to excluding certain conversions, such as between `int` and `GString`.
    ///
    // ASCII arsenal: / ╱ ⟋ ⧸ ⁄ ╱ ↗ ╲ \ ╲ ⟍ ⧹ ∖
    pub fn try_to_relaxed<T: FromGodot>(&self) -> Result<T, ConvertError> {
        try_from_variant_relaxed(self)
    }

    pub(crate) fn engine_try_to_relaxed<T: EngineFromGodot>(&self) -> Result<T, ConvertError> {
        try_from_variant_relaxed(self)
    }

    /// Helper function for relaxed variant conversion with panic on failure.
    /// Similar to [`to()`](Self::to) but uses relaxed conversion rules.
    pub(crate) fn to_relaxed_or_panic<T, F>(&self, context: F) -> T
    where
        T: EngineFromGodot,
        F: FnOnce() -> String,
    {
        self.engine_try_to_relaxed::<T>()
            .unwrap_or_else(|err| panic!("{}: {err}", context()))
    }

    /// Checks whether the variant is empty (`null` value in GDScript).
    ///
    /// See also [`get_type()`][Self::get_type].
    #[inline]
    pub fn is_nil(&self) -> bool {
        let sys_type = self.sys_type();
        if sys_type == sys::GDEXTENSION_VARIANT_TYPE_NIL {
            return true;
        }

        if sys_type == sys::GDEXTENSION_VARIANT_TYPE_OBJECT {
            return self.is_null_object();
        }

        false
    }

    /// Alias for [`is_nil()`][Self::is_nil].
    ///
    /// This method is provided for clarity when working with object-typed variants that may be `null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.is_nil()
    }

    /// Returns true if the variant currently holds a value of type `ty`.
    #[inline]
    pub fn is_type(&self, ty: VariantType) -> bool {
        self.get_type() == ty
    }

    /// Returns true if the variant holds an object.
    ///
    /// Alias for `self.is_type(VariantType::OBJECT)`.
    #[inline]
    pub fn is_object(&self) -> bool {
        self.is_type(VariantType::OBJECT)
    }

    /// Returns true if the variant holds an integer.
    #[inline]
    pub fn is_int(&self) -> bool {
        self.is_type(VariantType::INT)
    }

    /// Returns true if the variant holds a float.
    #[inline]
    pub fn is_float(&self) -> bool {
        self.is_type(VariantType::FLOAT)
    }

    /// Returns true if the variant holds a boolean.
    #[inline]
    pub fn is_bool(&self) -> bool {
        self.is_type(VariantType::BOOL)
    }

    /// Returns true if the variant holds a string.
    #[inline]
    pub fn is_string(&self) -> bool {
        self.is_type(VariantType::STRING)
    }

    /// Returns true if the variant holds an array.
    ///
    /// Alias for `self.is_type(VariantType::ARRAY)`.
    #[inline]
    pub fn is_array(&self) -> bool {
        self.is_type(VariantType::ARRAY)
    }

    /// Returns true if the variant holds a dictionary.
    ///
    /// Alias for `self.is_type(VariantType::DICTIONARY)`.
    #[inline]
    pub fn is_dictionary(&self) -> bool {
        self.is_type(VariantType::DICTIONARY)
    }

    /// Returns the variant as an `Array<Variant>`, or `Err` if it is not an array.
    #[inline]
    pub fn try_to_array(&self) -> Result<crate::builtin::Array<Variant>, ConvertError> {
        self.try_to()
    }

    /// Returns the variant as a `VarDictionary`, or `Err` if it is not a dictionary.
    #[inline]
    pub fn try_to_dictionary(&self) -> Result<crate::builtin::VarDictionary, ConvertError> {
        self.try_to()
    }

    /// Returns the variant as a `Gd<T>`, or `Err` if it is not an object of type `T`.
    #[inline]
    pub fn try_to_object<T: crate::obj::Inherits<crate::classes::Object> + crate::obj::GodotClass>(
        &self,
    ) -> Result<crate::obj::Gd<T>, ConvertError> {
        self.try_to()
    }

    /// Returns the variant as a `GString`, or `Err` if it is not a string.
    #[inline]
    pub fn try_to_gstring(&self) -> Result<GString, ConvertError> {
        self.try_to()
    }

    /// ⚠️ Returns the variant as an integer, using relaxed conversion rules, panicking if it fails.
    #[inline]
    pub fn to_int(&self) -> i64 {
        self.try_to_relaxed::<i64>()
            .unwrap_or_else(|err| panic!("Variant::to_int(): {err}"))
    }

    /// ⚠️ Returns the variant as a float, using relaxed conversion rules, panicking if it fails.
    #[inline]
    pub fn to_float(&self) -> f64 {
        self.try_to_relaxed::<f64>()
            .unwrap_or_else(|err| panic!("Variant::to_float(): {err}"))
    }

    /// ⚠️ Returns the variant as a boolean, using relaxed conversion rules, panicking if it fails.
    #[inline]
    pub fn to_bool(&self) -> bool {
        self.try_to_relaxed::<bool>()
            .unwrap_or_else(|err| panic!("Variant::to_bool(): {err}"))
    }

    /// ⚠️ Returns the variant as a `GString`, using relaxed conversion rules, panicking if it fails.
    #[inline]
    pub fn to_gstring(&self) -> GString {
        self.try_to_relaxed::<GString>()
            .unwrap_or_else(|err| panic!("Variant::to_gstring(): {err}"))
    }

    /// Returns the type that is currently held by this variant.
    ///
    /// Note that this returns `OBJECT` even if the variant holds a null object pointer. To check for
    /// null objects, use [`is_nil()`][Self::is_nil].
    #[inline]
    pub fn get_type(&self) -> VariantType {
        VariantType::from_sys(self.sys_type())
    }

    #[inline]
    fn is_null_object(&self) -> bool {
        // Faster check available from 4.4 onwards.
        #[cfg(since_api = "4.4")]
        return unsafe { interface_fn!(variant_get_object_instance_id)(self.var_sys()) == 0 };

        #[cfg(before_api = "4.4")]
        // SAFETY: caller verified that the raw type is OBJECT, so we can interpret the type-ptr as address of an object-ptr.
        unsafe {
            let object_ptr = crate::obj::raw_object_init(|type_ptr| {
                let converter = sys::builtin_fn!(object_from_variant);
                converter(type_ptr, sys::SysPtr::force_mut(self.var_sys()));
            });

            object_ptr.is_null()
        }
    }

    /// For variants holding an object, returns the object's instance ID.
    ///
    /// If the variant is not an object, returns `None`.
    ///
    /// # Panics
    /// If the variant holds an object and that object is dead.
    ///
    /// If you want to detect this case, use [`try_to::<Gd<...>>()`](Self::try_to). If you want to retrieve the previous instance ID of a
    /// freed object for whatever reason, use [`object_id_unchecked()`][Self::object_id_unchecked]. This method is only available from
    /// Godot 4.4 onwards.
    #[inline]
    pub fn object_id(&self) -> Option<crate::obj::InstanceId> {
        #[cfg(since_api = "4.4")]
        {
            assert!(
                self.get_type() != VariantType::OBJECT || self.is_object_alive(),
                "Variant::object_id(): object has been freed"
            );
            self.object_id_unchecked()
        }

        #[cfg(before_api = "4.4")]
        {
            use crate::meta::error::{ErrorKind, FromVariantError};
            match self.try_to::<crate::obj::Gd<crate::classes::Object>>() {
                Ok(obj) => Some(obj.instance_id_unchecked()),
                Err(c)
                    if matches!(
                        c.kind(),
                        ErrorKind::FromVariant(FromVariantError::DeadObject)
                    ) =>
                {
                    panic!("Variant::object_id(): object has been freed")
                }
                _ => None, // other conversion errors
            }
        }
    }

    /// For variants holding an object, returns the object's instance ID.
    ///
    /// If the variant is not an object, returns `None`.
    ///
    /// If the object is dead, the instance ID is still returned, similar to [`Gd::instance_id_unchecked()`][crate::obj::Gd::instance_id_unchecked].
    /// Unless you have a very good reason to use this, we recommend using [`object_id()`][Self::object_id] instead.
    #[cfg(since_api = "4.4")]
    pub fn object_id_unchecked(&self) -> Option<crate::obj::InstanceId> {
        // SAFETY: safe to call for non-object variants (returns 0).
        let raw_id: u64 = unsafe { interface_fn!(variant_get_object_instance_id)(self.var_sys()) };

        crate::obj::InstanceId::try_from_u64(raw_id)
    }

    /// ⚠️ Calls the specified `method` with the given `args`.
    ///
    /// Supports `Object` as well as built-ins with methods (e.g. `Array`, `Vector3`, `GString`, etc.).
    ///
    /// # Panics
    /// * If `self` is not a variant type which supports method calls.
    /// * If the method does not exist or the signature is not compatible with the passed arguments.
    /// * If the call causes an error.
    #[inline]
    pub fn call(&self, method: impl AsArg<StringName>, args: &[Variant]) -> Variant {
        arg_into_ref!(method);
        self.call_inner(method, args)
    }

    fn call_inner(&self, method: &StringName, args: &[Variant]) -> Variant {
        let mut error = sys::default_call_error();

        let mut args_stack = [ptr::null(); sys::MAX_STACK_ARGS];

        let (args_ptr, _args_heap) = if args.len() <= sys::MAX_STACK_ARGS {
            for (i, arg) in args.iter().enumerate() {
                args_stack[i] = arg.var_sys();
            }
            (args_stack.as_ptr(), Vec::new())
        } else {
            let v: Vec<_> = args.iter().map(|v| v.var_sys()).collect();
            (v.as_ptr(), v)
        };

        let result = unsafe {
            Variant::new_with_var_uninit(|variant_ptr| {
                interface_fn!(variant_call)(
                    sys::SysPtr::force_mut(self.var_sys()),
                    method.string_sys(),
                    args_ptr,
                    args.len() as i64,
                    variant_ptr,
                    ptr::addr_of_mut!(error),
                )
            })
        };

        if error.error != sys::GDEXTENSION_CALL_OK {
            let arg_types: Vec<_> = args.iter().map(Variant::get_type).collect();
            sys::panic_call_error(&error, "call", &arg_types);
        }
        result
    }

    /// Evaluates an expression using a GDScript operator.
    ///
    /// Returns the result of the operation, or `None` if the operation is not defined for the given operand types.
    ///
    /// Recommended to be used with fully-qualified call syntax.
    /// For example, `Variant::evaluate(&a, &b, VariantOperator::Add)` is equivalent to `a + b` in GDScript.
    pub fn evaluate(&self, rhs: &Variant, op: VariantOperator) -> Option<Variant> {
        use crate::obj::EngineEnum;

        let op_sys = op.ord() as sys::GDExtensionVariantOperator;
        let mut is_valid = false as u8;

        let result = unsafe {
            Self::new_with_var_uninit(|variant_ptr| {
                interface_fn!(variant_evaluate)(
                    op_sys,
                    self.var_sys(),
                    rhs.var_sys(),
                    variant_ptr,
                    ptr::addr_of_mut!(is_valid),
                )
            })
        };

        if is_valid == 1 {
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn sys_type(&self) -> sys::GDExtensionVariantType {
        unsafe {
            let ty: sys::GDExtensionVariantType = interface_fn!(variant_get_type)(self.var_sys());
            ty
        }
    }

    /// Gets the value of a key from this variant.
    ///
    /// # Panics
    /// If the operation is invalid for this variant type.
    pub fn get_keyed(&self, key: &Variant) -> Variant {
        let mut valid = false as sys::GDExtensionBool;
        let mut ret = Variant::nil();
        unsafe {
            interface_fn!(variant_get_keyed)(
                self.var_sys(),
                key.var_sys(),
                ret.var_sys_mut().cast(),
                ptr::addr_of_mut!(valid),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::get_keyed(): operation invalid"
        );
        ret
    }

    /// Sets the value of a key in this variant.
    ///
    /// # Panics
    /// If the operation is invalid for this variant type.
    pub fn set_keyed(&mut self, key: &Variant, value: &Variant) {
        let mut valid = false as sys::GDExtensionBool;
        unsafe {
            interface_fn!(variant_set_keyed)(
                self.var_sys_mut(),
                key.var_sys(),
                value.var_sys(),
                ptr::addr_of_mut!(valid),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::set_keyed(): operation invalid"
        );
    }

    /// Gets the value of a named key from this variant.
    ///
    /// # Panics
    /// If the operation is invalid for this variant type.
    pub fn get_named(&self, name: &StringName) -> Variant {
        let mut valid = false as sys::GDExtensionBool;
        let mut ret = Variant::nil();
        unsafe {
            interface_fn!(variant_get_named)(
                self.var_sys(),
                name.string_sys(),
                ret.var_sys_mut().cast(),
                ptr::addr_of_mut!(valid),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::get_named(): operation invalid"
        );
        ret
    }

    /// Sets the value of a named key in this variant.
    ///
    /// # Panics
    /// If the operation is invalid for this variant type.
    pub fn set_named(&mut self, name: &StringName, value: &Variant) {
        let mut valid = false as sys::GDExtensionBool;
        unsafe {
            interface_fn!(variant_set_named)(
                self.var_sys_mut(),
                name.string_sys(),
                value.var_sys(),
                ptr::addr_of_mut!(valid),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::set_named(): operation invalid"
        );
    }

    /// Gets the value at the specified index from this variant.
    ///
    /// # Panics
    /// * If the operation is invalid for this variant type.
    /// * If the index is out of bounds.
    pub fn get_indexed(&self, index: i64) -> Variant {
        let mut valid = false as sys::GDExtensionBool;
        let mut oob = false as sys::GDExtensionBool;
        let mut ret = Variant::nil();
        unsafe {
            interface_fn!(variant_get_indexed)(
                self.var_sys(),
                index,
                ret.var_sys_mut().cast(),
                ptr::addr_of_mut!(valid),
                ptr::addr_of_mut!(oob),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::get_indexed(): operation invalid"
        );
        assert!(
            !sys::conv::bool_from_sys(oob),
            "Variant::get_indexed(): index {index} out of bounds"
        );
        ret
    }

    /// Sets the value at the specified index in this variant.
    ///
    /// # Panics
    /// * If the operation is invalid for this variant type.
    /// * If the index is out of bounds.
    pub fn set_indexed(&mut self, index: i64, value: &Variant) {
        let mut valid = false as sys::GDExtensionBool;
        let mut oob = false as sys::GDExtensionBool;
        unsafe {
            interface_fn!(variant_set_indexed)(
                self.var_sys_mut(),
                index,
                value.var_sys(),
                ptr::addr_of_mut!(valid),
                ptr::addr_of_mut!(oob),
            );
        }
        assert!(
            sys::conv::bool_from_sys(valid),
            "Variant::set_indexed(): operation invalid"
        );
        assert!(
            !sys::conv::bool_from_sys(oob),
            "Variant::set_indexed(): index {index} out of bounds"
        );
    }

    /// ⚠️ Gets the value of a key and converts it to `T`, panicking on failure.
    pub fn at_as<K: ToGodot, T: FromGodot>(&self, key: K) -> T {
        self.get_keyed(&key.to_variant()).to::<T>()
    }

    /// Gets the value of a key and converts it to `T` (fallible).
    pub fn get_as<K: ToGodot, T: FromGodot>(&self, key: K) -> Option<T> {
        let key = key.to_variant();
        let mut valid = false as sys::GDExtensionBool;
        let mut ret = Variant::nil();
        unsafe {
            interface_fn!(variant_get_keyed)(
                self.var_sys(),
                key.var_sys(),
                ret.var_sys_mut().cast(),
                ptr::addr_of_mut!(valid),
            );
        }
        if sys::conv::bool_from_sys(valid) {
            ret.try_to::<T>().ok()
        } else {
            None
        }
    }

    /// Return Godot's string representation of the variant.
    ///
    /// See also `Display` impl.
    #[allow(unused_mut)] // result
    pub fn stringify(&self) -> GString {
        let mut result = GString::new();
        unsafe {
            interface_fn!(variant_stringify)(self.var_sys(), result.string_sys_mut());
        }
        result
    }

    /// Return Godot's hash value for the variant.
    ///
    /// _Godot equivalent : `@GlobalScope.hash()`_
    pub fn hash_u32(&self) -> u32 {
        // @GlobalScope.hash() actually calls the VariantUtilityFunctions::hash(&Variant) function (C++).
        // This function calls the passed reference's `hash` method, which returns a uint32_t.
        // Therefore, casting this function to u32 is always fine.
        unsafe { interface_fn!(variant_hash)(self.var_sys()) }
            .try_into()
            .expect("Godot hashes are uint32_t")
    }

    /// Interpret the `Variant` as `bool`.
    ///
    /// Returns `false` only if the variant's current value is the default value for its type. For example:
    /// - `nil` for the nil type
    /// - `false` for bool
    /// - zero for numeric types
    /// - empty string
    /// - empty container (array, packed array, dictionary)
    /// - default-constructed other builtins (e.g. zero vector, degenerate plane, zero RID, etc...)
    #[inline]
    pub fn booleanize(&self) -> bool {
        // See Variant::is_zero(), roughly https://github.com/godotengine/godot/blob/master/core/variant/variant.cpp#L859.

        unsafe { interface_fn!(variant_booleanize)(self.var_sys()) != 0 }
    }

    /// Assuming that this is of type `OBJECT`, checks whether the object is dead.
    ///
    /// Does not check again that the variant has type `OBJECT`.
    pub(crate) fn is_object_alive(&self) -> bool {
        sys::strict_assert_eq!(self.get_type(), VariantType::OBJECT);

        crate::global::is_instance_valid(self)

        // In case there are ever problems with this approach, alternative implementation:
        // self.stringify() != "<Freed Object>".into()
    }

    // Conversions from/to Godot C++ `Variant*` pointers
    ffi_methods! {
        type sys::GDExtensionVariantPtr = *mut Self;

        fn new_from_var_sys = new_from_sys;
        fn new_with_var_uninit = new_with_uninit;
        fn new_with_var_init = new_with_init;
        fn var_sys = sys;
        fn var_sys_mut = sys_mut;
    }
}

// All manually implemented unsafe functions on `Variant`.
// Deny `unsafe_op_in_unsafe_fn` so we don't forget to check safety invariants.
#[doc(hidden)]
#[deny(unsafe_op_in_unsafe_fn)]
impl Variant {
    /// Moves this variant into a variant sys pointer. This is the same as using [`GodotFfi::move_return_ptr`].
    ///
    /// # Safety
    ///
    /// `dst` must be a valid variant pointer.
    pub(crate) unsafe fn move_into_var_ptr(self, dst: sys::GDExtensionVariantPtr) {
        let dst: sys::GDExtensionTypePtr = dst.cast();
        // SAFETY: `dst` is a valid Variant pointer. Additionally `Variant` doesn't behave differently for `Standard` and `Virtual`
        // pointer calls.
        unsafe {
            self.move_return_ptr(dst, sys::PtrcallType::Standard);
        }
    }

    /// Fallible construction of a `Variant` using a fallible initialization function.
    ///
    /// # Safety
    ///
    /// If `init_fn` returns `Ok(())`, then it must have initialized the pointer passed to it in accordance with [`GodotFfi::new_with_uninit`].
    #[doc(hidden)]
    pub unsafe fn new_with_var_uninit_result<E>(
        init_fn: impl FnOnce(sys::GDExtensionUninitializedVariantPtr) -> Result<(), E>,
    ) -> Result<Self, E> {
        // Relies on current macro expansion of from_var_sys_init() having a certain implementation.

        let mut raw = std::mem::MaybeUninit::<Variant>::uninit();

        let var_uninit_ptr =
            raw.as_mut_ptr() as <sys::GDExtensionVariantPtr as sys::SysPtr>::Uninit;

        // SAFETY: `map` only runs the provided closure for the `Ok(())` variant, in which case `raw` has definitely been initialized.
        init_fn(var_uninit_ptr).map(|_success| unsafe { raw.assume_init() })
    }

    /// Convert a `Variant` sys pointer to a reference to a `Variant`.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live `Variant` for the duration of `'a`.
    pub(crate) unsafe fn borrow_var_sys<'a>(ptr: sys::GDExtensionConstVariantPtr) -> &'a Variant {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        // SAFETY: `ptr` is a pointer to a live `Variant` for the duration of `'a`.
        unsafe { &*(ptr.cast::<Variant>()) }
    }

    /// Convert an array of `Variant` sys pointers to a slice of `Variant` references all with unbounded lifetimes.
    ///
    /// # Safety
    ///
    /// Either `variant_ptr_array` is null, or it must be safe to call [`std::slice::from_raw_parts`] with
    /// `variant_ptr_array` cast to `*const &'a Variant` and `length`.
    pub(crate) unsafe fn borrow_ref_slice<'a>(
        variant_ptr_array: *const sys::GDExtensionConstVariantPtr,
        length: usize,
    ) -> &'a [&'a Variant] {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        // Godot may pass null to signal "no arguments" (e.g. in custom callables).
        if variant_ptr_array.is_null() {
            Self::strict_ensure_zero_length(length);
            return &[];
        }

        // Note: Raw pointers and references have the same memory layout.
        // See https://doc.rust-lang.org/reference/type-layout.html#pointers-and-references-layout.
        let variant_ptr_array = variant_ptr_array.cast::<&Variant>();

        // SAFETY: `variant_ptr_array` isn't null so it is safe to call `from_raw_parts` on the pointer cast to `*const &Variant`.
        unsafe { std::slice::from_raw_parts(variant_ptr_array, length) }
    }

    /// Convert an array of `Variant` sys pointers to a slice with unbounded lifetime.
    ///
    /// # Safety
    ///
    /// Either `variant_array` is null, or it must be safe to call [`std::slice::from_raw_parts`] with
    /// `variant_array` cast to `*const Variant` and `length`.
    pub(crate) unsafe fn borrow_slice<'a>(
        variant_array: sys::GDExtensionConstVariantPtr,
        length: usize,
    ) -> &'a [Variant] {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        // Godot may pass null to signal "no arguments" (e.g. in custom callables).
        if variant_array.is_null() {
            Self::strict_ensure_zero_length(length);
            return &[];
        }

        let variant_array = variant_array.cast::<Variant>();

        // SAFETY: `variant_array` isn't null so it is safe to call `from_raw_parts` on the pointer cast to `*const Variant`.
        unsafe { std::slice::from_raw_parts(variant_array, length) }
    }

    /// Convert an array of `Variant` sys pointers to a mutable slice with unbounded lifetime.
    ///
    /// # Safety
    ///
    /// Either `variant_array` is null, or it must be safe to call [`std::slice::from_raw_parts_mut`] with
    /// `variant_array` cast to `*mut Variant` and `length`.
    pub(crate) unsafe fn borrow_slice_mut<'a>(
        variant_array: sys::GDExtensionVariantPtr,
        length: usize,
    ) -> &'a mut [Variant] {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        // Godot may pass null to signal "no arguments" (e.g. in custom callables).
        if variant_array.is_null() {
            Self::strict_ensure_zero_length(length);
            return &mut [];
        }

        let variant_array = variant_array.cast::<Variant>();

        // SAFETY: `variant_array` isn't null so it is safe to call `from_raw_parts_mut` on the pointer cast to `*mut Variant`.
        unsafe { std::slice::from_raw_parts_mut(variant_array, length) }
    }

    fn strict_ensure_zero_length(_length: usize) {
        sys::strict_assert_eq!(
            _length,
            0,
            "Variant::borrow_slice*(): pointer is null but length is not 0"
        );
    }

    /// Consumes self and turns it into a sys-ptr, should be used together with [`from_owned_var_sys`](Self::from_owned_var_sys).
    ///
    /// This will leak memory unless `from_owned_var_sys` is called on the returned pointer.
    pub(crate) fn into_owned_var_sys(self) -> sys::GDExtensionVariantPtr {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        let leaked = Box::into_raw(Box::new(self));
        leaked.cast()
    }

    /// Creates a `Variant` from a sys-ptr without incrementing the refcount.
    ///
    /// # Safety
    ///
    /// * Must only be used on a pointer returned from a call to [`into_owned_var_sys`](Self::into_owned_var_sys).
    /// * Must not be called more than once on the same pointer.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub(crate) unsafe fn from_owned_var_sys(ptr: sys::GDExtensionVariantPtr) -> Self {
        sys::static_assert_eq_size_align!(Variant, sys::types::OpaqueVariant);

        let ptr = ptr.cast::<Self>();

        // SAFETY: `ptr` was returned from a call to `into_owned_var_sys`, which means it was created by a call to
        // `Box::into_raw`, thus we can use `Box::from_raw` here. Additionally, this is only called once on this pointer.
        let boxed = unsafe { Box::from_raw(ptr) };
        *boxed
    }
}

impl ArrayElement for Variant {}

// SAFETY:
// `from_opaque` properly initializes a dereferenced pointer to an `OpaqueVariant`.
// `std::mem::swap` is sufficient for returning a value.
unsafe impl GodotFfi for Variant {
    const VARIANT_TYPE: ExtVariantType = ExtVariantType::Variant;

    ffi_methods! { type sys::GDExtensionTypePtr = *mut Self; .. }
}

crate::meta::impl_godot_as_self!(Variant: ByRef);

impl Default for Variant {
    fn default() -> Self {
        unsafe {
            Self::new_with_var_uninit(|variant_ptr| {
                interface_fn!(variant_new_nil)(variant_ptr);
            })
        }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        unsafe {
            Self::new_with_var_uninit(|variant_ptr| {
                interface_fn!(variant_new_copy)(variant_ptr, self.var_sys());
            })
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            interface_fn!(variant_destroy)(self.var_sys_mut());
        }
    }
}

// Variant is not Eq because it can contain floats and other types composed of floats.
impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        Self::evaluate(self, other, VariantOperator::EQUAL) //.
            .is_some_and(|v| v.to::<bool>())
        // If there is no defined conversion (-> None), then they are non-equal.
    }
}

macro_rules! impl_variant_partial_eq_int {
    ($($ty:ty),*) => {
        $(
            impl PartialEq<$ty> for Variant {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    match self.get_type() {
                        VariantType::INT => self.to_int() == *other as i64,
                        VariantType::FLOAT => self.to_float() == *other as f64,
                        _ => false,
                    }
                }
            }

            impl PartialEq<Variant> for $ty {
                #[inline]
                fn eq(&self, other: &Variant) -> bool {
                    other.eq(self)
                }
            }
        )*
    };
}

impl_variant_partial_eq_int!(i64, i32, i16, i8, u32, u16, u8);

impl PartialEq<f64> for Variant {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        match self.get_type() {
            VariantType::INT => self.to_int() as f64 == *other,
            VariantType::FLOAT => self.to_float() == *other,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for f64 {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<f32> for Variant {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        match self.get_type() {
            VariantType::INT => self.to_int() as f64 == *other as f64,
            VariantType::FLOAT => self.to_float() == *other as f64,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for f32 {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<bool> for Variant {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        if self.is_bool() {
            return self.to_bool() == *other;
        }
        false
    }
}

impl PartialEq<Variant> for bool {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<GString> for Variant {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        match self.get_type() {
            VariantType::STRING => self.to::<GString>() == *other,
            VariantType::STRING_NAME => self.to::<StringName>() == *other,
            VariantType::NODE_PATH => self.to::<NodePath>() == *other,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for GString {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<StringName> for Variant {
    #[inline]
    fn eq(&self, other: &StringName) -> bool {
        match self.get_type() {
            VariantType::STRING => self.to::<GString>() == *other,
            VariantType::STRING_NAME => self.to::<StringName>() == *other,
            VariantType::NODE_PATH => self.to::<NodePath>() == *other,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for StringName {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<NodePath> for Variant {
    #[inline]
    fn eq(&self, other: &NodePath) -> bool {
        match self.get_type() {
            VariantType::STRING => self.to::<GString>() == *other,
            VariantType::STRING_NAME => self.to::<StringName>() == *other,
            VariantType::NODE_PATH => self.to::<NodePath>() == *other,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for NodePath {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl<T: crate::obj::GodotClass> PartialEq<crate::obj::Gd<T>> for Variant {
    #[inline]
    fn eq(&self, other: &crate::obj::Gd<T>) -> bool {
        if self.is_object() {
            if let Some(id) = self.object_id() {
                return id == other.instance_id();
            }
        }
        false
    }
}

impl<T: crate::obj::GodotClass> PartialEq<Variant> for crate::obj::Gd<T> {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl<T: crate::obj::GodotClass> PartialEq<Option<crate::obj::Gd<T>>> for Variant {
    #[inline]
    fn eq(&self, other: &Option<crate::obj::Gd<T>>) -> bool {
        match other {
            Some(gd) => self.eq(gd),
            None => self.is_nil(),
        }
    }
}

impl<T: crate::obj::GodotClass> PartialEq<Variant> for Option<crate::obj::Gd<T>> {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<crate::builtin::Callable> for Variant {
    #[inline]
    fn eq(&self, other: &crate::builtin::Callable) -> bool {
        if self.is_type(VariantType::CALLABLE) {
            return self.to::<crate::builtin::Callable>() == *other;
        }
        false
    }
}

impl PartialEq<Variant> for crate::builtin::Callable {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&str> for Variant {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        match self.get_type() {
            VariantType::STRING => self.to::<GString>() == *other,
            VariantType::STRING_NAME => self.to::<StringName>() == *other,
            VariantType::NODE_PATH => self.to::<NodePath>() == *other,
            _ => false,
        }
    }
}

impl PartialEq<Variant> for &str {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialEq<String> for Variant {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.eq(&other.as_str())
    }
}

impl PartialEq<Variant> for String {
    #[inline]
    fn eq(&self, other: &Variant) -> bool {
        other.eq(self)
    }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        if self
            .evaluate(other, VariantOperator::LESS)
            .is_some_and(|v| v.to::<bool>())
        {
            return Some(std::cmp::Ordering::Less);
        }

        if self
            .evaluate(other, VariantOperator::GREATER)
            .is_some_and(|v| v.to::<bool>())
        {
            return Some(std::cmp::Ordering::Greater);
        }

        None
    }
}

macro_rules! impl_variant_bin_op {
    ($trait:ident, $method:ident, $op:expr, $op_str:expr) => {
        impl std::ops::$trait for Variant {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self::Output {
                self.evaluate(&rhs, $op).unwrap_or_else(|| {
                    panic!(
                        "Variant operator {} failed between {:?} and {:?}",
                        $op_str,
                        self.get_type(),
                        rhs.get_type()
                    )
                })
            }
        }

        impl std::ops::$trait<&Variant> for Variant {
            type Output = Self;
            fn $method(self, rhs: &Variant) -> Self::Output {
                self.evaluate(rhs, $op).unwrap_or_else(|| {
                    panic!(
                        "Variant operator {} failed between {:?} and {:?}",
                        $op_str,
                        self.get_type(),
                        rhs.get_type()
                    )
                })
            }
        }
    };
}

macro_rules! impl_variant_assign_op {
    ($trait:ident, $method:ident, $op:expr, $op_str:expr) => {
        impl std::ops::$trait for Variant {
            fn $method(&mut self, rhs: Self) {
                *self = self.evaluate(&rhs, $op).unwrap_or_else(|| {
                    panic!(
                        "Variant operator {} failed between {:?} and {:?}",
                        $op_str,
                        self.get_type(),
                        rhs.get_type()
                    )
                });
            }
        }

        impl std::ops::$trait<&Variant> for Variant {
            fn $method(&mut self, rhs: &Variant) {
                *self = self.evaluate(rhs, $op).unwrap_or_else(|| {
                    panic!(
                        "Variant operator {} failed between {:?} and {:?}",
                        $op_str,
                        self.get_type(),
                        rhs.get_type()
                    )
                });
            }
        }
    };
}

impl_variant_bin_op!(Add, add, VariantOperator::ADD, "+");
impl_variant_bin_op!(Sub, sub, VariantOperator::SUBTRACT, "-");
impl_variant_bin_op!(Mul, mul, VariantOperator::MULTIPLY, "*");
impl_variant_bin_op!(Div, div, VariantOperator::DIVIDE, "/");
impl_variant_bin_op!(Rem, rem, VariantOperator::MODULO, "%");

impl_variant_assign_op!(AddAssign, add_assign, VariantOperator::ADD, "+=");
impl_variant_assign_op!(SubAssign, sub_assign, VariantOperator::SUBTRACT, "-=");
impl_variant_assign_op!(MulAssign, mul_assign, VariantOperator::MULTIPLY, "*=");
impl_variant_assign_op!(DivAssign, div_assign, VariantOperator::DIVIDE, "/=");
impl_variant_assign_op!(RemAssign, rem_assign, VariantOperator::MODULO, "%=");

impl std::ops::Neg for Variant {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let from_type = self.get_type();
        self.evaluate(&Variant::nil(), VariantOperator::NEGATE)
            .unwrap_or_else(|| panic!("Variant unary operator - failed for {from_type:?}"))
    }
}

impl std::ops::Not for Variant {
    type Output = Self;
    fn not(self) -> Self::Output {
        let from_type = self.get_type();
        let op = if from_type == VariantType::BOOL {
            VariantOperator::NOT
        } else {
            VariantOperator::BIT_NEGATE
        };
        let op_str = if from_type == VariantType::BOOL {
            "!"
        } else {
            "~"
        };

        self.evaluate(&Variant::nil(), op)
            .unwrap_or_else(|| panic!("Variant unary operator {op_str} failed for {from_type:?}"))
    }
}

impl_variant_bin_op!(BitAnd, bitand, VariantOperator::BIT_AND, "&");
impl_variant_bin_op!(BitOr, bitor, VariantOperator::BIT_OR, "|");
impl_variant_bin_op!(BitXor, bitxor, VariantOperator::BIT_XOR, "^");
impl_variant_bin_op!(Shl, shl, VariantOperator::SHIFT_LEFT, "<<");
impl_variant_bin_op!(Shr, shr, VariantOperator::SHIFT_RIGHT, ">>");

impl_variant_assign_op!(BitAndAssign, bitand_assign, VariantOperator::BIT_AND, "&=");
impl_variant_assign_op!(BitOrAssign, bitor_assign, VariantOperator::BIT_OR, "|=");
impl_variant_assign_op!(BitXorAssign, bitxor_assign, VariantOperator::BIT_XOR, "^=");
impl_variant_assign_op!(ShlAssign, shl_assign, VariantOperator::SHIFT_LEFT, "<<=");
impl_variant_assign_op!(ShrAssign, shr_assign, VariantOperator::SHIFT_RIGHT, ">>=");

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.stringify();
        write!(f, "{s}")
    }
}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_type() {
            // Special case for arrays: avoids converting to VarArray (the only Array type in VariantDispatch),
            // which fails for typed arrays and causes a panic. This can cause an infinite loop with Debug, or abort.
            // Can be removed if there's ever a "possibly typed" Array type (e.g. AnyArray) in the library.
            VariantType::ARRAY => {
                // SAFETY: type is checked, and only operation is print (out data flow, no covariant in access).
                let array = unsafe { VarArray::from_variant_unchecked(self) };
                array.fmt(f)
            }

            // Converting to objects before printing causes their refcount to increment, leading to an Observer effect
            // where `Debug` actually changes the object statistics. As such, fetch information without instantiating Gd<T>.
            VariantType::OBJECT => classes::debug_string_variant(self, f, "VariantGd"),

            // VariantDispatch also includes dead objects via `FreedObject` enumerator, which maps to "<Freed Object>".
            _ => VariantDispatch::from_variant(self).fmt(f),
        }
    }
}

fn try_from_variant_relaxed<T: EngineFromGodot>(variant: &Variant) -> Result<T, ConvertError> {
    let from_type = variant.get_type();
    let to_type = match ffi_variant_type::<T>() {
        ExtVariantType::Variant => {
            // Converting to Variant always succeeds.
            return T::engine_try_from_variant(variant);
        }
        ExtVariantType::Concrete(to_type) if from_type == to_type => {
            // If types are the same, use the regular conversion.
            // This is both an optimization (avoids more FFI) and ensures consistency between strict and relaxed conversions for identical types.
            return T::engine_try_from_variant(variant);
        }
        ExtVariantType::Concrete(to_type) => to_type,
    };

    // Non-NIL types can technically be converted to NIL according to `variant_can_convert_strict()`, however that makes no sense -- from
    // neither a type perspective (NIL is unit, not never type), nor a practical one. Disallow any such conversions.
    if to_type == VariantType::NIL || !can_convert_godot_strict(from_type, to_type) {
        return Err(FromVariantError::BadType {
            expected: to_type,
            actual: from_type,
        }
        .into_error(variant.clone()));
    }

    // Find correct from->to conversion constructor.
    let converter = get_variant_to_type_constructor(to_type);

    // Must be available, since we checked with `variant_can_convert_strict`.
    let converter =
        converter.unwrap_or_else(|| panic!("missing converter for {from_type:?} -> {to_type:?}"));

    // Perform actual conversion on the FFI types. The GDExtension conversion constructor only works with types supported
    // by Godot (i.e. GodotType), not GodotConvert (like i8).
    let ffi_result = unsafe {
        <<T::Via as GodotType>::Ffi as GodotFfi>::new_with_uninit(|result_ptr| {
            converter(result_ptr, sys::SysPtr::force_mut(variant.var_sys()));
        })
    };

    // Try to convert the FFI types back to the user type. Can still fail, e.g. i64 -> i8.
    let via = <T::Via as GodotType>::try_from_ffi(ffi_result)?;
    let concrete = T::engine_try_from_godot(via)?;

    Ok(concrete)
}

fn get_variant_to_type_constructor(
    to_type: VariantType,
) -> sys::GDExtensionTypeFromVariantConstructorFunc {
    static CONSTRUCTORS: sys::Global<[sys::GDExtensionTypeFromVariantConstructorFunc; 40]> =
        sys::Global::new(|| [None; 40]);

    let index = to_type.sys() as usize;
    if index >= 40 {
        return unsafe { interface_fn!(get_variant_to_type_constructor)(to_type.sys()) };
    }

    let mut constructors = CONSTRUCTORS.lock();
    if let Some(c) = constructors[index] {
        return Some(c);
    }

    let c = unsafe { interface_fn!(get_variant_to_type_constructor)(to_type.sys()) };
    constructors[index] = c;
    c
}

#[cfg(since_api = "4.4")]
pub(crate) fn get_variant_get_internal_ptr_func(
    variant_type: VariantType,
) -> sys::GDExtensionVariantGetInternalPtrFunc {
    static GETTERS: sys::Global<[sys::GDExtensionVariantGetInternalPtrFunc; 40]> =
        sys::Global::new(|| [None; 40]);

    let index = variant_type.sys() as usize;
    if index >= 40 {
        return unsafe { interface_fn!(variant_get_ptr_internal_getter)(variant_type.sys()) };
    }

    let mut getters = GETTERS.lock();
    if let Some(g) = getters[index] {
        return Some(g);
    }

    let g = unsafe { interface_fn!(variant_get_ptr_internal_getter)(variant_type.sys()) };
    getters[index] = g;
    g
}

fn can_convert_godot_strict(from_type: VariantType, to_type: VariantType) -> bool {
    // Godot "strict" conversion is still quite permissive.
    // See Variant::can_convert_strict() in C++, https://github.com/godotengine/godot/blob/master/core/variant/variant.cpp#L532-L532.
    unsafe {
        let can_convert_fn = interface_fn!(variant_can_convert_strict);
        can_convert_fn(from_type.sys(), to_type.sys()) == sys::conv::SYS_TRUE
    }
}
