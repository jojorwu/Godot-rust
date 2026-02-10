/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi::VariantType;

use crate::builtin::{GString, StringName, VarDictionary};
use crate::global::{PropertyHint, PropertyUsageFlags};
use crate::meta::{element_godot_type_name, ArrayElement, ClassId, GodotType, PackedArrayElement};
use crate::obj::{bounds, Bounds, EngineBitfield, EngineEnum, GodotClass, Inherits};
use crate::registry::class::get_dyn_property_hint_string;
use crate::registry::property::{Export, Var};
use crate::{classes, sys};

/// Describes a property's type, name and metadata for Godot.
///
/// `PropertyInfo` is used throughout the Godot binding to describe properties, method parameters and return types.
///
/// This is a high-level abstraction over the low-level FFI type `sys::GDExtensionPropertyInfo`. Unlike the FFI version which only stores
/// pointers, `PropertyInfo` owns its data, ensuring it remains valid for the lifetime of the struct.
///
/// See also [`MethodInfo`](crate::meta::MethodInfo) for describing method signatures and [`ClassId`] for type-IDs of Godot classes.
///
/// # Construction
/// For most use cases, prefer the convenience constructors:
/// - [`new_var::<T>()`](Self::new_var) -- creates property info for a `#[var]` attribute.
/// - [`new_export::<T>()`](Self::new_export) -- for an `#[export]` attribute.
/// - [`new_group()`](Self::new_group) / [`new_subgroup()`](Self::new_subgroup) -- for editor groups.
///
/// # Example
/// ```no_run
/// use godot::meta::{PropertyInfo, PropertyHintInfo, ClassId};
/// use godot::builtin::{StringName, VariantType};
/// use godot::global::PropertyUsageFlags;
///
/// // Integer property without a specific class
/// let count_property = PropertyInfo {
///     variant_type: VariantType::INT,
///     class_id: ClassId::none(),  // Only OBJECT types need a real class ID.
///     property_name: StringName::from("count"),
///     hint_info: PropertyHintInfo::none(),
///     usage: PropertyUsageFlags::DEFAULT,
/// };
/// ```
///
/// Here, `class_id` is set to [`ClassId::none()`] because integer properties do not require a specific class. For objects, you can use
/// [`ClassId::new_cached::<T>(...)`][ClassId::new_cached] if you have a Rust type, or [`ClassId::new_dynamic(...)`][ClassId::new_dynamic]
/// for dynamic contexts and script classes.
#[derive(Clone, Debug)]
// Note: is not #[non_exhaustive], so adding fields is a breaking change. Mostly used internally at the moment though.
// Note: There was an idea of a high-level representation of the following, but it's likely easier and more efficient to use introspection
// APIs like `is_array_of_elem()`, unless there's a real user-facing need.
// pub(crate) enum SimplePropertyType {
//     Variant { ty: VariantType },
//     Array { elem_ty: VariantType },
//     Object { class_id: ClassId },
// }
pub struct PropertyInfo {
    /// Type of the property.
    ///
    /// For objects, this should be set to [`VariantType::OBJECT`] and use the `class_id` field to specify the actual class.  \
    /// For generic [`Variant`](crate::builtin::Variant) properties, use [`VariantType::NIL`].
    pub variant_type: VariantType,

    /// The specific class identifier for object-typed properties in Godot.
    ///
    /// This is only relevant when `variant_type` is set to [`VariantType::OBJECT`].
    ///
    /// # Example
    /// ```no_run
    /// use godot::meta::ClassId;
    /// use godot::classes::Node3D;
    /// use godot::obj::GodotClass; // Trait method ::class_id().
    ///
    /// let none_id = ClassId::none();                      // For built-ins (not classes).
    /// let static_id = Node3D::class_id();                 // For classes with a Rust type.
    /// let dynamic_id = ClassId::new_dynamic("MyScript");  // For runtime class names.
    /// ```
    pub class_id: ClassId,

    /// The name of this property as it appears in Godot's object system.
    pub property_name: StringName,

    /// Additional type information and validation constraints for this property.
    ///
    /// Use functions from [`export_info_functions`](crate::registry::property::export_info_functions) to create common hints,
    /// or [`PropertyHintInfo::none()`] for no hints.
    ///
    /// See [`PropertyHintInfo`] struct in Rust, as well as [`PropertyHint`] in the official Godot documentation.
    ///
    /// [`PropertyHint`]: https://docs.godotengine.org/en/latest/classes/class_%40globalscope.html#enum-globalscope-propertyhint
    pub hint_info: PropertyHintInfo,

    /// Flags controlling how this property should be used and displayed by the Godot engine.
    ///
    /// Common values:
    /// - [`PropertyUsageFlags::DEFAULT`] -- standard property (readable, writable, saved, appears in editor).
    /// - [`PropertyUsageFlags::STORAGE`] -- persisted, but not shown in editor.
    /// - [`PropertyUsageFlags::EDITOR`] -- shown in editor, but not persisted.
    ///
    /// See also [`PropertyUsageFlags`] in the official Godot documentation for a complete list of flags.
    ///
    /// [`PropertyUsageFlags`]: https://docs.godotengine.org/en/latest/classes/class_%40globalscope.html#enum-globalscope-propertyusageflags
    pub usage: PropertyUsageFlags,
}

impl Default for PropertyInfo {
    fn default() -> Self {
        Self {
            variant_type: VariantType::NIL,
            class_id: ClassId::none(),
            property_name: StringName::default(),
            hint_info: PropertyHintInfo::none(),
            usage: PropertyUsageFlags::DEFAULT,
        }
    }
}

impl PropertyInfo {
    /// Create a `PropertyInfo` from a dictionary.
    pub fn from_dictionary(dict: &VarDictionary) -> Self {
        use crate::obj::EngineEnum;

        let variant_type = dict
            .get_as::<&str, i64>("type")
            .map(|ty| VariantType::from_sys(ty as sys::GDExtensionVariantType))
            .unwrap_or(VariantType::NIL);

        let property_name = dict.get_as::<&str, StringName>("name").unwrap_or_default();

        let class_id = dict
            .get_as::<&str, StringName>("class_name")
            .map(|name| ClassId::new_dynamic(name.to_string()))
            .unwrap_or(ClassId::none());

        let hint = dict
            .get_as::<&str, i64>("hint")
            .map(|h| PropertyHint::from_ord(h as i32))
            .unwrap_or(PropertyHint::NONE);

        let hint_string = dict
            .get_as::<&str, GString>("hint_string")
            .unwrap_or_default();

        let usage = dict
            .get_as::<&str, i64>("usage")
            .map(|u| PropertyUsageFlags::from_ord(u as u64))
            .unwrap_or(PropertyUsageFlags::DEFAULT);

        Self {
            variant_type,
            class_id,
            property_name,
            hint_info: PropertyHintInfo { hint, hint_string },
            usage,
        }
    }

    /// Convert `PropertyInfo` to a dictionary.
    pub fn to_dictionary(&self) -> VarDictionary {
        use crate::builtin::vdict;
        use crate::obj::EngineEnum;

        vdict! {
            "type": self.variant_type.ord() as i64,
            "name": self.property_name.clone(),
            "class_name": self.class_id.to_string_name(),
            "hint": self.hint_info.hint.ord() as i64,
            "hint_string": self.hint_info.hint_string.clone(),
            "usage": self.usage.ord() as i64,
        }
    }

    /// Create a new `PropertyInfo` representing a property named `property_name` with type `T` automatically.
    ///
    /// This will generate property info equivalent to what a `#[var]` attribute would produce.
    pub fn new_var<T: Var>(property_name: impl AsRef<str>) -> Self {
        T::Via::property_info(property_name.as_ref()).with_hint_info(T::var_hint())
    }

    /// Create a new `PropertyInfo` for an exported property named `property_name` with type `T` automatically.
    ///
    /// This will generate property info equivalent to what an `#[export]` attribute would produce.
    pub fn new_export<T: Export>(property_name: impl AsRef<str>) -> Self {
        T::Via::property_info(property_name.as_ref()).with_hint_info(T::export_hint())
    }

    /// Create a new `PropertyInfo` for an object of type `T`.
    pub fn new_object<T: GodotClass>(property_name: impl Into<StringName>) -> Self {
        Self {
            variant_type: VariantType::OBJECT,
            class_id: T::class_id(),
            property_name: property_name.into(),
            ..Self::default()
        }
    }

    /// Create a new `PropertyInfo` for a resource of type `T`.
    ///
    /// This also sets the hint to [`PropertyHint::RESOURCE_TYPE`].
    pub fn new_resource<T>(property_name: impl Into<StringName>) -> Self
    where
        T: GodotClass + Inherits<classes::Resource>,
    {
        Self::new_object::<T>(property_name).with_hint_info(PropertyHintInfo {
            hint: PropertyHint::RESOURCE_TYPE,
            hint_string: T::class_id().to_gstring(),
        })
    }

    /// Returns a copy of this `PropertyInfo` with the given `usage`.
    pub fn with_usage(self, usage: PropertyUsageFlags) -> Self {
        Self { usage, ..self }
    }

    /// Returns a copy of this `PropertyInfo` with the given `class_id`.
    pub fn with_class(self, class_id: ClassId) -> Self {
        Self { class_id, ..self }
    }

    /// Returns a copy of this `PropertyInfo` with the given `variant_type`.
    pub fn with_variant_type(self, variant_type: VariantType) -> Self {
        Self {
            variant_type,
            ..self
        }
    }

    /// Change the `hint` and `hint_string` to be the given `hint_info`.
    ///
    /// See [`export_info_functions`](crate::registry::property::export_info_functions) for functions that return appropriate `PropertyHintInfo`s for
    /// various Godot annotations.
    ///
    /// # Example
    /// Creating an `@export_range` property.
    ///
    /// ```no_run
    /// use godot::meta::{PropertyInfo, PropertyHintInfo};
    ///
    /// let property = PropertyInfo::new_export::<f64>("my_range_property")
    ///     .with_hint_info(PropertyHintInfo::range(0.0, 10.0).with_step(0.1).with_suffix("mm"));
    /// ```
    pub fn with_hint_info(self, hint_info: PropertyHintInfo) -> Self {
        Self { hint_info, ..self }
    }

    /// Returns a copy of this `PropertyInfo` with the `READ_ONLY` flag set.
    pub fn read_only(mut self) -> Self {
        self.usage = self.usage.with_flag(PropertyUsageFlags::READ_ONLY, true);
        self
    }

    /// Returns a copy of this `PropertyInfo` with the `EDITOR` flag cleared.
    pub fn no_editor(mut self) -> Self {
        self.usage = self.usage.with_flag(PropertyUsageFlags::EDITOR, false);
        self
    }

    /// Returns a copy of this `PropertyInfo` with the `STORAGE` flag cleared.
    pub fn no_storage(mut self) -> Self {
        self.usage = self.usage.with_flag(PropertyUsageFlags::STORAGE, false);
        self
    }

    /// Returns a copy of this `PropertyInfo` with both `EDITOR` and `STORAGE` flags set.
    ///
    /// This is the same as the default [`PropertyUsageFlags::DEFAULT`].
    pub fn persisted(mut self) -> Self {
        self.usage = self
            .usage
            .with_flag(PropertyUsageFlags::EDITOR, true)
            .with_flag(PropertyUsageFlags::STORAGE, true);
        self
    }

    /// Returns a copy of this `PropertyInfo` with the given `flag` set to `value`.
    pub fn with_usage_flag(mut self, flag: PropertyUsageFlags, value: bool) -> Self {
        self.usage = self.usage.with_flag(flag, value);
        self
    }

    /// Sets the property hint to a range.
    pub fn range(self, min: f64, max: f64) -> Self {
        self.with_hint_info(PropertyHintInfo::range(min, max))
    }

    /// Sets the property hint to an enum.
    pub fn enum_names(self, names: &[&str]) -> Self {
        self.with_hint_info(PropertyHintInfo::enum_names(names))
    }

    /// Sets the property hint to bit flags.
    pub fn flags(self, names: &[&str]) -> Self {
        self.with_hint_info(PropertyHintInfo::flags(names))
    }

    /// Sets the property hint to a file path.
    pub fn file(self, filter: &str) -> Self {
        self.with_hint_info(PropertyHintInfo::file(filter))
    }

    /// Sets the property hint to a directory path.
    pub fn dir(self) -> Self {
        self.with_hint_info(PropertyHintInfo::dir())
    }

    /// Sets the property hint to a multiline string.
    pub fn multiline(self) -> Self {
        self.with_hint_info(PropertyHintInfo::multiline())
    }

    /// Sets the step for a range hint.
    pub fn with_step(self, step: f64) -> Self {
        let hint_info = self.hint_info.clone().with_step(step);
        self.with_hint_info(hint_info)
    }

    /// Sets the suffix for a range hint.
    pub fn with_suffix(self, suffix: &str) -> Self {
        let hint_info = self.hint_info.clone().with_suffix(suffix);
        self.with_hint_info(hint_info)
    }

    /// Create a new `PropertyInfo` representing a group in Godot.
    ///
    /// See [`EditorInspector`](https://docs.godotengine.org/en/latest/classes/class_editorinspector.html#class-editorinspector) in Godot for
    /// more information.
    pub fn new_group(group_name: &str, group_prefix: &str) -> Self {
        Self {
            variant_type: VariantType::NIL,
            class_id: ClassId::none(),
            property_name: group_name.into(),
            hint_info: PropertyHintInfo {
                hint: PropertyHint::NONE,
                hint_string: group_prefix.into(),
            },
            usage: PropertyUsageFlags::GROUP,
        }
    }

    /// Create a new `PropertyInfo` representing a subgroup in Godot.
    ///
    /// See [`EditorInspector`](https://docs.godotengine.org/en/latest/classes/class_editorinspector.html#class-editorinspector) in Godot for
    /// more information.
    pub fn new_subgroup(subgroup_name: &str, subgroup_prefix: &str) -> Self {
        Self {
            variant_type: VariantType::NIL,
            class_id: ClassId::none(),
            property_name: subgroup_name.into(),
            hint_info: PropertyHintInfo {
                hint: PropertyHint::NONE,
                hint_string: subgroup_prefix.into(),
            },
            usage: PropertyUsageFlags::SUBGROUP,
        }
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------
    // Introspection API -- could be made public in the future

    pub(crate) fn is_array_of_elem<T>(&self) -> bool
    where
        T: ArrayElement,
    {
        self.variant_type == VariantType::ARRAY
            && self.hint_info.hint == PropertyHint::ARRAY_TYPE
            && self.hint_info.hint_string == T::Via::godot_type_name()
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------
    // FFI conversion functions

    /// Converts to the FFI type. Keep this object allocated while using that!
    #[doc(hidden)]
    pub fn property_sys(&self) -> sys::GDExtensionPropertyInfo {
        use crate::obj::{EngineBitfield as _, EngineEnum as _};

        sys::GDExtensionPropertyInfo {
            type_: self.variant_type.sys(),
            name: sys::SysPtr::force_mut(self.property_name.string_sys()),
            class_name: sys::SysPtr::force_mut(self.class_id.string_sys()),
            hint: u32::try_from(self.hint_info.hint.ord()).expect("hint.ord()"),
            hint_string: sys::SysPtr::force_mut(self.hint_info.hint_string.string_sys()),
            usage: u32::try_from(self.usage.ord()).expect("usage.ord()"),
        }
    }

    #[doc(hidden)]
    pub fn empty_sys() -> sys::GDExtensionPropertyInfo {
        use crate::obj::{EngineBitfield as _, EngineEnum as _};

        sys::GDExtensionPropertyInfo {
            type_: VariantType::NIL.sys(),
            name: std::ptr::null_mut(),
            class_name: std::ptr::null_mut(),
            hint: PropertyHint::NONE.ord() as u32,
            hint_string: std::ptr::null_mut(),
            usage: PropertyUsageFlags::NONE.ord() as u32,
        }
    }

    /// Consumes self and turns it into a `sys::GDExtensionPropertyInfo`, should be used together with
    /// [`free_owned_property_sys`](Self::free_owned_property_sys).
    ///
    /// This will leak memory unless used together with `free_owned_property_sys`.
    pub(crate) fn into_owned_property_sys(self) -> sys::GDExtensionPropertyInfo {
        use crate::obj::{EngineBitfield as _, EngineEnum as _};

        sys::GDExtensionPropertyInfo {
            type_: self.variant_type.sys(),
            name: self.property_name.into_owned_string_sys(),
            class_name: sys::SysPtr::force_mut(self.class_id.string_sys()),
            hint: u32::try_from(self.hint_info.hint.ord()).expect("hint.ord()"),
            hint_string: self.hint_info.hint_string.into_owned_string_sys(),
            usage: u32::try_from(self.usage.ord()).expect("usage.ord()"),
        }
    }

    /// Properly frees a `sys::GDExtensionPropertyInfo` created by [`into_owned_property_sys`](Self::into_owned_property_sys).
    ///
    /// # Safety
    ///
    /// * Must only be used on a struct returned from a call to `into_owned_property_sys`, without modification.
    /// * Must not be called more than once on a `sys::GDExtensionPropertyInfo` struct.
    pub(crate) unsafe fn free_owned_property_sys(info: sys::GDExtensionPropertyInfo) {
        // SAFETY: This function was called on a pointer returned from `into_owned_property_sys`, thus both `info.name` and
        // `info.hint_string` were created from calls to `into_owned_string_sys` on their respective types.
        // Additionally, this function isn't called more than once on a struct containing the same `name` or `hint_string` pointers.
        unsafe {
            let _name = StringName::from_owned_string_sys(info.name);
            let _hint_string = GString::from_owned_string_sys(info.hint_string);
        }
    }

    /// Moves its values into given `GDExtensionPropertyInfo`, dropping previous values if necessary.
    ///
    /// # Safety
    ///
    /// * `property_info_ptr` must be valid.
    ///
    pub(crate) unsafe fn move_into_property_info_ptr(
        self,
        property_info_ptr: *mut sys::GDExtensionPropertyInfo,
    ) {
        let ptr = &mut *property_info_ptr;

        ptr.usage = u32::try_from(self.usage.ord()).expect("usage.ord()");
        ptr.hint = u32::try_from(self.hint_info.hint.ord()).expect("hint.ord()");
        ptr.type_ = self.variant_type.sys();

        *StringName::borrow_string_sys_mut(ptr.name) = self.property_name;
        *GString::borrow_string_sys_mut(ptr.hint_string) = self.hint_info.hint_string;

        if self.class_id != ClassId::none() {
            *StringName::borrow_string_sys_mut(ptr.class_name) = self.class_id.to_string_name();
        }
    }

    /// Creates copy of given `sys::GDExtensionPropertyInfo`.
    ///
    /// # Safety
    ///
    /// * `property_info_ptr` must be valid.
    pub(crate) unsafe fn new_from_sys(
        property_info_ptr: *mut sys::GDExtensionPropertyInfo,
    ) -> Self {
        let ptr = *property_info_ptr;

        Self {
            variant_type: VariantType::from_sys(ptr.type_),
            class_id: ClassId::none(),
            property_name: StringName::new_from_string_sys(ptr.name),
            hint_info: PropertyHintInfo {
                hint: PropertyHint::from_ord(ptr.hint.to_owned() as i32),
                hint_string: GString::new_from_string_sys(ptr.hint_string),
            },
            usage: PropertyUsageFlags::from_ord(ptr.usage as u64),
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Info needed by Godot, for how to export a type to the editor.
///
/// Property hints provide extra metadata about the property, such as:
/// - Range constraints for numeric values.
/// - Enum value lists.
/// - File/directory paths.
/// - Resource types.
/// - Array element types.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PropertyHintInfo {
    pub hint: PropertyHint,
    pub hint_string: GString,
}

impl Default for PropertyHintInfo {
    fn default() -> Self {
        Self::none()
    }
}

impl PropertyHintInfo {
    /// Create a new `PropertyHintInfo` with a property hint of [`PROPERTY_HINT_NONE`](PropertyHint::NONE), and no hint string.
    pub fn none() -> Self {
        Self {
            hint: PropertyHint::NONE,
            hint_string: GString::new(),
        }
    }

    /// Create a new `PropertyHintInfo` for a range.
    pub fn range(min: f64, max: f64) -> Self {
        Self {
            hint: PropertyHint::RANGE,
            hint_string: (&format!("{min},{max}")).into(),
        }
    }

    /// Create a new `PropertyHintInfo` for an enum.
    pub fn enum_names(names: &[&str]) -> Self {
        Self {
            hint: PropertyHint::ENUM,
            hint_string: (&names.join(",")).into(),
        }
    }

    /// Create a new `PropertyHintInfo` for bit flags.
    pub fn flags(names: &[&str]) -> Self {
        Self {
            hint: PropertyHint::FLAGS,
            hint_string: (&names.join(",")).into(),
        }
    }

    /// Returns a copy of this `PropertyHintInfo` with the given `step`.
    ///
    /// This method only has an effect if the hint is [`PropertyHint::RANGE`].
    /// It expects the hint string to follow Godot's standard range format: `"min,max[,step[,extra]]"`.
    pub fn with_step(mut self, step: f64) -> Self {
        if self.hint != PropertyHint::RANGE {
            return self;
        }

        let hint_str = self.hint_string.to_string();
        let mut parts: Vec<String> = hint_str.split(',').map(String::from).collect();

        if parts.len() >= 2 {
            let step_str = step.to_string();
            if parts.len() == 2 {
                parts.push(step_str);
            } else {
                parts[2] = step_str;
            }
            self.hint_string = GString::from(parts.join(",").as_str());
        }
        self
    }

    /// Returns a copy of this `PropertyHintInfo` with the given `suffix`.
    pub fn with_suffix(mut self, suffix: &str) -> Self {
        let mut s = self.hint_string.to_string();
        if !s.is_empty() {
            s.push(',');
        }
        s.push_str("suffix:");
        s.push_str(suffix);
        self.hint_string = (&s).into();
        self
    }

    /// Create a new `PropertyHintInfo` for a file path.
    pub fn file(filter: &str) -> Self {
        Self {
            hint: PropertyHint::FILE,
            hint_string: filter.into(),
        }
    }

    /// Create a new `PropertyHintInfo` for a directory path.
    pub fn dir() -> Self {
        Self {
            hint: PropertyHint::DIR,
            hint_string: GString::new(),
        }
    }

    /// Create a new `PropertyHintInfo` for a multiline string.
    pub fn multiline() -> Self {
        Self {
            hint: PropertyHint::MULTILINE_TEXT,
            hint_string: GString::new(),
        }
    }

    /// Use [`PROPERTY_HINT_NONE`](PropertyHint::NONE) with `T`'s Godot type name.
    ///
    /// Starting with Godot version 4.3, the hint string will always be the empty string. Before that, the hint string is set to
    /// be the Godot type name of `T`.
    pub fn type_name<T: GodotType>() -> Self {
        let type_name = T::godot_type_name();
        let hint_string = if sys::GdextBuild::since_api("4.3") {
            GString::new()
        } else {
            GString::from(&type_name)
        };

        Self {
            hint: PropertyHint::NONE,
            hint_string,
        }
    }

    /// Use for `#[var]` properties -- [`PROPERTY_HINT_ARRAY_TYPE`](PropertyHint::ARRAY_TYPE) with the type name as hint string.
    pub fn var_array_element<T: ArrayElement>() -> Self {
        Self {
            hint: PropertyHint::ARRAY_TYPE,
            hint_string: GString::from(&element_godot_type_name::<T>()),
        }
    }

    /// Use for `#[export]` properties -- [`PROPERTY_HINT_TYPE_STRING`](PropertyHint::TYPE_STRING) with the **element** type string as hint string.
    pub fn export_array_element<T: ArrayElement>() -> Self {
        Self {
            hint: PropertyHint::TYPE_STRING,
            hint_string: GString::from(&T::element_type_string()),
        }
    }

    /// Use for `#[export]` properties -- [`PROPERTY_HINT_TYPE_STRING`](PropertyHint::TYPE_STRING) with the **element** type string as hint string.
    pub fn export_packed_array_element<T: PackedArrayElement>() -> Self {
        Self {
            hint: PropertyHint::TYPE_STRING,
            hint_string: GString::from(&T::element_type_string()),
        }
    }

    pub fn export_gd<T>() -> Self
    where
        T: GodotClass + Bounds<Exportable = bounds::Yes>,
    {
        let hint = if T::inherits::<classes::Resource>() {
            PropertyHint::RESOURCE_TYPE
        } else if T::inherits::<classes::Node>() {
            PropertyHint::NODE_TYPE
        } else {
            unreachable!("classes not inheriting from Resource or Node should not be exportable")
        };

        // Godot does this by default too; the hint is needed when the class is a resource/node,
        // but doesn't seem to make a difference otherwise.
        let hint_string = T::class_id().to_gstring();

        Self { hint, hint_string }
    }

    pub fn export_dyn_gd<T, D>() -> Self
    where
        T: GodotClass + Bounds<Exportable = bounds::Yes>,
        D: ?Sized + 'static,
    {
        PropertyHintInfo {
            hint_string: GString::from(&get_dyn_property_hint_string::<T, D>()),
            ..PropertyHintInfo::export_gd::<T>()
        }
    }

    #[doc(hidden)]
    pub fn object_as_node_class<T>() -> Option<ClassId>
    where
        T: GodotClass + Bounds<Exportable = bounds::Yes>,
    {
        T::inherits::<classes::Node>().then(|| T::class_id())
    }
}
