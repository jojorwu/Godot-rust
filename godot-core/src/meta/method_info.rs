/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi::conv::u32_to_usize;

use crate::builtin::{StringName, VarDictionary, Variant};
use crate::global::MethodFlags;
use crate::meta::{AsArg, ClassId, PropertyInfo, ToGodot};
use crate::obj::EngineBitfield;
use crate::sys;

/// Describes a method's signature and metadata required by the Godot engine.
///
/// Primarily used when implementing custom script instances via the [`ScriptInstance`][crate::obj::script::ScriptInstance] trait.
/// It contains metadata Godot needs to describe and call a method.
///
/// `MethodInfo` is a high-level abstraction over the low-level FFI type `sys::GDExtensionMethodInfo`.
///
/// See also [`PropertyInfo`] for describing individual method parameters and return types.
///
/// # Example
/// ```no_run
/// use godot::meta::{MethodInfo, PropertyInfo, PropertyHintInfo, ClassId};
/// use godot::builtin::{StringName, Variant, VariantType};
/// use godot::global::{MethodFlags, PropertyUsageFlags};
/// use godot::classes::Node2D;
/// use godot::obj::GodotClass; // Trait method ::class_id().
///
/// // Describe a Godot method (`World` is a GDScript class):
/// //   func spawn_at(world: World, position: Vector2) -> Node2D.
/// let method = MethodInfo {
///     id: 0,
///     method_name: StringName::from("spawn_at"),
///     class_id: ClassId::none(),
///     return_type: PropertyInfo {
///         variant_type: VariantType::OBJECT,
///         class_id: Node2D::class_id(),
///         property_name: StringName::default(), // Return types use empty string.
///         hint_info: PropertyHintInfo::none(),
///         usage: PropertyUsageFlags::DEFAULT,
///     },
///     arguments: vec![
///         PropertyInfo {
///             variant_type: VariantType::OBJECT,
///             class_id: ClassId::new_dynamic("World"),
///             property_name: StringName::from("world"),
///             hint_info: PropertyHintInfo::none(),
///             usage: PropertyUsageFlags::DEFAULT,
///         },
///         PropertyInfo {
///             variant_type: VariantType::VECTOR2,
///             class_id: ClassId::none(),
///             property_name: StringName::from("position"),
///             hint_info: PropertyHintInfo::none(),
///             usage: PropertyUsageFlags::DEFAULT,
///         },
///     ],
///     default_arguments: vec![],
///     flags: MethodFlags::DEFAULT,
/// };
/// ```
#[derive(Clone, Debug)]
pub struct MethodInfo {
    /// Unique identifier for the method within its class.
    ///
    /// This ID can be used to distinguish between methods and is typically set by the implementation. For script instances,
    /// this is often just a sequential index.
    pub id: i32,

    /// The name of the method, as it appears in Godot.
    pub method_name: StringName,

    /// The class this method belongs to.
    ///
    /// For script-defined methods, this is typically the script's class ID obtained via [`ClassId::new_dynamic()`].
    /// Use [`ClassId::none()`] if the class is not applicable or unknown.
    pub class_id: ClassId,

    /// Description of the method's return type.
    ///
    /// See [`PropertyInfo`] for how to construct type information. For methods that
    /// don't return a value (void), use `VariantType::NIL`.
    pub return_type: PropertyInfo,

    /// Descriptions of each method parameter.
    ///
    /// Each element describes one parameter's type, name, and metadata. The order
    /// matches the parameter order in the method signature.
    pub arguments: Vec<PropertyInfo>,

    /// Default values for parameters with defaults.
    ///
    /// Contains the actual default [`Variant`] values for parameters that have them.
    /// The length of this vector is typically less than or equal to `arguments.len()`,
    /// containing defaults only for trailing parameters.
    pub default_arguments: Vec<Variant>,

    /// Method flags controlling behavior and access.
    ///
    /// See [`MethodFlags`] for available options like `NORMAL`, `VIRTUAL`, `CONST`, etc.
    pub flags: MethodFlags,
}

impl MethodInfo {
    /// Create a `MethodInfo` from a dictionary.
    pub fn from_dictionary(dict: &VarDictionary) -> Self {
        use crate::builtin::VarArray;
        use crate::obj::EngineBitfield;

        let method_name = dict
            .get_as::<&str, StringName>("name")
            .unwrap_or_default();

        let id = dict.get_as::<&str, i64>("id").unwrap_or(0) as i32;

        let return_type = dict
            .get_as::<&str, VarDictionary>("return")
            .map(|d| PropertyInfo::from_dictionary(&d))
            .unwrap_or_default();

        let arguments = dict
            .get_as::<&str, VarArray>("args")
            .map(|arr: VarArray| {
                arr.iter_shared()
                    .map(|v| PropertyInfo::from_dictionary(&v.to::<VarDictionary>()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let default_arguments = dict
            .get_as::<&str, VarArray>("default_args")
            .map(|arr: VarArray| arr.iter_shared().collect::<Vec<_>>())
            .unwrap_or_default();

        let flags = dict
            .get_as::<&str, i64>("flags")
            .map(|f| MethodFlags::from_ord(f as u64))
            .unwrap_or(MethodFlags::DEFAULT);

        Self {
            id,
            method_name,
            class_id: ClassId::none(), // Class ID usually not in the dict.
            return_type,
            arguments,
            default_arguments,
            flags,
        }
    }

    /// Convert `MethodInfo` to a dictionary.
    pub fn to_dictionary(&self) -> VarDictionary {
        use crate::builtin::{vdict, VarArray};
        use crate::obj::EngineBitfield;

        let args: VarArray = self
            .arguments
            .iter()
            .map(|arg| arg.to_dictionary().to_variant())
            .collect();

        let default_args: VarArray = self.default_arguments.iter().cloned().collect();

        vdict! {
            "name": self.method_name.clone(),
            "args": args,
            "default_args": default_args,
            "return": self.return_type.to_dictionary(),
            "flags": self.flags.ord() as i64,
            "id": self.id as i64,
        }
    }

    /// Creates a new `MethodInfo` with the given name.
    pub fn new(method_name: impl AsArg<StringName>) -> Self {
        Self {
            id: 0,
            method_name: method_name.into_arg().to_owned(),
            class_id: ClassId::none(),
            return_type: PropertyInfo::default(),
            arguments: vec![],
            default_arguments: vec![],
            flags: MethodFlags::DEFAULT,
        }
    }

    /// Sets the method's unique ID.
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the class ID this method belongs to.
    pub fn with_class_id(mut self, class_id: ClassId) -> Self {
        self.class_id = class_id;
        self
    }

    /// Sets the method's return type.
    pub fn with_return_type(mut self, return_type: PropertyInfo) -> Self {
        self.return_type = return_type;
        self
    }

    /// Adds a parameter to the method signature.
    pub fn with_argument(mut self, argument: PropertyInfo) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Sets all method parameters.
    pub fn with_arguments(mut self, arguments: Vec<PropertyInfo>) -> Self {
        self.arguments = arguments;
        self
    }

    /// Adds a default value for the last parameter.
    pub fn with_default_argument(mut self, default_argument: impl ToGodot) -> Self {
        self.default_arguments.push(default_argument.to_variant());
        self
    }

    /// Sets all default arguments.
    pub fn with_default_arguments(mut self, default_arguments: Vec<Variant>) -> Self {
        self.default_arguments = default_arguments;
        self
    }

    /// Sets method flags.
    pub fn with_flags(mut self, flags: MethodFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the `VARARG` flag.
    pub fn vararg(mut self) -> Self {
        self.flags = self.flags.with_flag(MethodFlags::VARARG, true);
        self
    }

    /// Sets the `STATIC` flag.
    pub fn static_method(mut self) -> Self {
        self.flags = self.flags.with_flag(MethodFlags::STATIC, true);
        self
    }

    /// Sets the `VIRTUAL` flag.
    pub fn virtual_method(mut self) -> Self {
        self.flags = self.flags.with_flag(MethodFlags::VIRTUAL, true);
        self
    }

    /// Sets the `CONST` flag.
    pub fn const_method(mut self) -> Self {
        self.flags = self.flags.with_flag(MethodFlags::CONST, true);
        self
    }

    /// Sets the `EDITOR` flag.
    pub fn editor_method(mut self) -> Self {
        self.flags = self.flags.with_flag(MethodFlags::EDITOR, true);
        self
    }

    /// Consumes self and turns it into a `sys::GDExtensionMethodInfo`, should be used together with
    /// [`free_owned_method_sys`](Self::free_owned_method_sys).
    ///
    /// This will leak memory unless used together with `free_owned_method_sys`.
    #[doc(hidden)]
    pub fn into_owned_method_sys(self) -> sys::GDExtensionMethodInfo {
        use crate::obj::EngineBitfield as _;

        // Destructure self to ensure all fields are used.
        let Self {
            id,
            method_name,
            class_id: _class_id,
            return_type,
            arguments,
            default_arguments,
            flags,
        } = self;

        let argument_count: u32 = arguments
            .len()
            .try_into()
            .expect("cannot have more than `u32::MAX` arguments");
        let arguments = arguments
            .into_iter()
            .map(|arg| arg.into_owned_property_sys())
            .collect::<Box<[_]>>();
        let arguments = Box::leak(arguments).as_mut_ptr();

        let default_argument_count: u32 = default_arguments
            .len()
            .try_into()
            .expect("cannot have more than `u32::MAX` default arguments");
        let default_argument = default_arguments
            .into_iter()
            .map(|arg| arg.into_owned_var_sys())
            .collect::<Box<[_]>>();
        let default_arguments = Box::leak(default_argument).as_mut_ptr();

        sys::GDExtensionMethodInfo {
            id,
            name: method_name.into_owned_string_sys(),
            return_value: return_type.into_owned_property_sys(),
            argument_count,
            arguments,
            default_argument_count,
            default_arguments,
            flags: flags.ord().try_into().expect("flags should be valid"),
        }
    }

    /// Properly frees a `sys::GDExtensionMethodInfo` created by [`into_owned_method_sys`](Self::into_owned_method_sys).
    ///
    /// # Safety
    ///
    /// * Must only be used on a struct returned from a call to `into_owned_method_sys`, without modification.
    /// * Must not be called more than once on a `sys::GDExtensionMethodInfo` struct.
    #[doc(hidden)]
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe fn free_owned_method_sys(info: sys::GDExtensionMethodInfo) {
        // Destructure info to ensure all fields are used.
        let sys::GDExtensionMethodInfo {
            name,
            return_value,
            flags: _flags,
            id: _id,
            argument_count,
            arguments,
            default_argument_count,
            default_arguments,
        } = info;

        // SAFETY: `name` is a pointer that was returned from `StringName::into_owned_string_sys`, and has not been freed before this.
        let _name = unsafe { StringName::from_owned_string_sys(name) };

        // SAFETY: `return_value` is a pointer that was returned from `PropertyInfo::into_owned_property_sys`, and has not been freed before
        // this.
        unsafe { PropertyInfo::free_owned_property_sys(return_value) };

        // SAFETY:
        // - `from_raw_parts_mut`: `arguments` comes from `as_mut_ptr()` on a mutable slice of length `argument_count`, and no other
        //    accesses to the pointer happens for the lifetime of the slice.
        // - `Box::from_raw`: The slice was returned from a call to `Box::leak`, and we have ownership of the value behind this pointer.
        let arguments = unsafe {
            let slice = std::slice::from_raw_parts_mut(arguments, u32_to_usize(argument_count));

            Box::from_raw(slice)
        };

        for info in arguments.iter() {
            // SAFETY: These infos were originally created from a call to `PropertyInfo::into_owned_property_sys`, and this method
            // will not be called again on this pointer.
            unsafe { PropertyInfo::free_owned_property_sys(*info) }
        }

        // SAFETY:
        // - `from_raw_parts_mut`: `default_arguments` comes from `as_mut_ptr()` on a mutable slice of length `default_argument_count`, and no
        //    other accesses to the pointer happens for the lifetime of the slice.
        // - `Box::from_raw`: The slice was returned from a call to `Box::leak`, and we have ownership of the value behind this pointer.
        let default_arguments = unsafe {
            let slice = std::slice::from_raw_parts_mut(
                default_arguments,
                u32_to_usize(default_argument_count),
            );

            Box::from_raw(slice)
        };

        for variant in default_arguments.iter() {
            // SAFETY: These pointers were originally created from a call to `Variant::into_owned_var_sys`, and this method will not be
            // called again on this pointer.
            let _variant = unsafe { Variant::from_owned_var_sys(*variant) };
        }
    }
}
