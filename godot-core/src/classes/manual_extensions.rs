/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Adds new convenience APIs to existing classes.
//!
//! This should not add new functionality, but provide existing one in a slightly nicer way to use. Generally, we should be conservative
//! about adding methods here, as it's a potentially endless quest, and many are better suited in high-level APIs or third-party crates.
//!
//! See also sister module [super::type_safe_replacements].

use crate::builtin::{GString, NodePath, StringName, Variant};
use crate::classes::{Node, Object, PackedScene, Resource, ResourceLoader, ResourceSaver};
use crate::meta::{arg_into_ref, AsArg, FromGodot, PropertyInfo, ToGodot};
use crate::obj::{Gd, Inherits};

/// Error returned by [`Node::try_get_node_as`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetNodeError {
    /// No node was found at the given path.
    NotFound,
    /// A node was found, but it could not be cast to the target type.
    BadType {
        actual: StringName,
        expected: StringName,
    },
}

impl std::fmt::Display for GetNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "node not found"),
            Self::BadType { actual, expected } => {
                write!(f, "expected type {expected}, but found {actual}")
            }
        }
    }
}

impl std::error::Error for GetNodeError {}

/// Manual extensions for the `Object` class.
impl Object {
    /// ⚠️ Retrieves a property value, panicking if not found or cannot be converted to `T`.
    pub fn get_as<T: FromGodot>(&self, property: impl AsArg<StringName>) -> T {
        arg_into_ref!(property);
        self.try_get_as::<T>(property).unwrap_or_else(|| {
            panic!(
                "Object::get_as(): property '{property}' not found or cannot be converted to {to}",
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves a property value (fallible).
    pub fn try_get_as<T: FromGodot>(&self, property: impl AsArg<StringName>) -> Option<T> {
        self.get(property).try_to::<T>().ok()
    }

    /// Sets a property value.
    pub fn set_as<T: ToGodot>(&mut self, property: impl AsArg<StringName>, value: T) {
        self.set(property, &value.to_variant());
    }

    /// ⚠️ Retrieves a metadata value, panicking if not found or cannot be converted to `T`.
    pub fn get_meta_as<T: FromGodot>(&self, name: impl AsArg<StringName>) -> T {
        arg_into_ref!(name);
        self.try_get_meta_as::<T>(name).unwrap_or_else(|| {
            panic!(
                "Object::get_meta_as(): meta '{name}' not found or cannot be converted to {to}",
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves a metadata value (fallible).
    pub fn try_get_meta_as<T: FromGodot>(&self, name: impl AsArg<StringName>) -> Option<T> {
        self.get_meta(name).try_to::<T>().ok()
    }

    /// Sets a metadata value.
    pub fn set_meta_as<T: ToGodot>(&mut self, name: impl AsArg<StringName>, value: T) {
        self.set_meta(name, &value.to_variant());
    }

    /// ⚠️ Calls a method and converts the return value to `T`, panicking if it fails.
    pub fn call_as<T: FromGodot>(
        &mut self,
        method: impl AsArg<StringName>,
        args: &[Variant],
    ) -> T {
        arg_into_ref!(method);
        self.try_call_as::<T>(method, args).unwrap_or_else(|| {
            panic!(
                "Object::call_as(): method '{method}' call failed or cannot convert return value to {to}",
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Calls a method and converts the return value to `T` (fallible).
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

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `Node` class.
impl Node {
    /// ⚠️ Retrieves the node at path `path`, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the node is not found, or if it does not have type `T` or inherited.
    pub fn get_node_as<T>(&self, path: impl AsArg<NodePath>) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(path);

        match self.try_get_node_as::<T>(path) {
            Ok(node) => node,
            Err(err) => panic!("Node::get_node_as(): {err} at path `{path}`"),
        }
    }

    /// Retrieves the node at path `path` (fallible).
    ///
    /// If the node is not found, or if it does not have type `T` or inherited,
    /// a [`GetNodeError`] will be returned.
    pub fn try_get_node_as<T>(&self, path: impl AsArg<NodePath>) -> Result<Gd<T>, GetNodeError>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(path);

        match self.get_node_or_null(path) {
            Some(node) => node.try_cast::<T>().map_err(|node| GetNodeError::BadType {
                actual: node.dynamic_class_string(),
                expected: T::class_id().to_string_name(),
            }),
            None => Err(GetNodeError::NotFound),
        }
    }

    /// Retrieves the node at path `path`, returning `None` if not found or bad type.
    pub fn get_node_or_none<T>(&self, path: impl AsArg<NodePath>) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.try_get_node_as::<T>(path).ok()
    }

    /// Alias for [`get_node_as()`][Self::get_node_as].
    pub fn get_node_typed<T>(&self, path: impl AsArg<NodePath>) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_node_as::<T>(path)
    }

    /// ⚠️ Retrieves the parent node, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the parent is not found, or if it does not have type `T` or inherited.
    pub fn get_parent_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_parent_as::<T>()
            .unwrap_or_else(|| panic!("Node::get_parent_as(): parent not found or bad type"))
    }

    /// Retrieves the parent node (fallible).
    ///
    /// If the parent is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    pub fn try_get_parent_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_parent().and_then(|parent| parent.try_cast::<T>().ok())
    }

    /// ⚠️ Retrieves the owner node, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the owner is not found, or if it does not have type `T` or inherited.
    pub fn get_owner_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_owner_as::<T>()
            .unwrap_or_else(|| panic!("Node::get_owner_as(): owner not found or bad type"))
    }

    /// Retrieves the owner node (fallible).
    ///
    /// If the owner is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    pub fn try_get_owner_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_owner().and_then(|owner| owner.try_cast::<T>().ok())
    }

    /// ⚠️ Retrieves the child node at index `index`, panicking if out of bounds or bad type.
    ///
    /// # Panics
    /// If `index` is out of bounds, or if the node does not have type `T` or inherited.
    pub fn get_child_as<T>(&self, index: usize) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_child_as::<T>(index)
            .unwrap_or_else(|| panic!("Node::get_child_as(): index {index} out of bounds or bad type"))
    }

    /// Retrieves the child node at index `index` (fallible).
    ///
    /// If `index` is out of bounds, or if the node does not have type `T` or inherited,
    /// `None` will be returned.
    pub fn try_get_child_as<T>(&self, index: usize) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_child(index as i32)
            .and_then(|node| node.try_cast::<T>().ok())
    }

    /// Alias for [`get_child_as()`][Self::get_child_as].
    pub fn get_child_typed<T>(&self, index: usize) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_child_as::<T>(index)
    }

    /// Retrieves all children, cast to type `T`.
    ///
    /// Children that cannot be cast to `T` are ignored.
    pub fn get_children_as<T>(&self) -> Vec<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_children()
            .iter_shared()
            .filter_map(|node| node.try_cast::<T>().ok())
            .collect()
    }

    /// Finds the first child whose name matches `pattern`, cast to type `T`.
    ///
    /// If no child is found or if it cannot be cast to `T`, `None` is returned.
    pub fn find_child_as<T>(
        &self,
        pattern: impl AsArg<GString>,
        recursive: bool,
        owned: bool,
    ) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.find_child_ex(pattern)
            .recursive(recursive)
            .owned(owned)
            .done()
            .and_then(|node| node.try_cast::<T>().ok())
    }

    /// Alias for [`find_child_as()`][Self::find_child_as].
    pub fn find_child_typed<T>(
        &self,
        pattern: impl AsArg<GString>,
        recursive: bool,
        owned: bool,
    ) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.find_child_as::<T>(pattern, recursive, owned)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `ClassDB` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::ClassDB {
    /// ⚠️ Instantiates a class by name, panicking if it cannot be created or bad type.
    pub fn instantiate_as<T>(&self, class: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(class);
        self.try_instantiate_as::<T>(class).unwrap_or_else(|| {
            panic!(
                "ClassDB::instantiate_as(): failed to instantiate {class} as {to}",
                to = T::class_id()
            )
        })
    }

    /// Instantiates a class by name (fallible).
    pub fn try_instantiate_as<T>(&self, class: impl AsArg<StringName>) -> Option<Gd<T>>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(class);
        self.instantiate(class)
            .and_then(|obj| obj.try_cast::<T>().ok())
    }

    /// Alias for [`instantiate_as()`][Self::instantiate_as].
    pub fn instantiate_typed<T>(&self, class: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        self.instantiate_as::<T>(class)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `EditorInterface` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::EditorInterface {
    /// Retrieves the editor main screen control, cast to type `T`.
    pub fn get_editor_main_screen_as<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_editor_main_screen().cast::<T>()
    }

    /// Retrieves the editor base control, cast to type `T`.
    pub fn get_base_control_as<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_base_control().cast::<T>()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `ProjectSettings` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::ProjectSettings {
    /// ⚠️ Retrieves a setting value, panicking if not found or cannot be converted to `T`.
    ///
    /// # Panics
    /// If the setting is not found, or if its value cannot be converted to `T`.
    pub fn get_setting_as<T: FromGodot>(&self, name: impl AsArg<GString>) -> T {
        self.try_get_setting_as::<T>(name)
            .unwrap_or_else(|| panic!("ProjectSettings::get_setting_as(): setting not found or wrong type"))
    }

    /// Retrieves a setting value (fallible).
    ///
    /// If the setting is not found, or if its value cannot be converted to `T`,
    /// `None` will be returned.
    pub fn try_get_setting_as<T: FromGodot>(&self, name: impl AsArg<GString>) -> Option<T> {
        self.get_setting(name).try_to::<T>().ok()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `Engine` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::Engine {
    /// ⚠️ Retrieves a singleton instance by name, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the singleton is not found, or if it does not have type `T` or inherited.
    pub fn get_singleton_as<T>(&self, name: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        self.try_get_singleton_as::<T>(name)
            .unwrap_or_else(|| panic!("Engine::get_singleton_as(): singleton not found or bad type"))
    }

    /// Retrieves a singleton instance by name (fallible).
    ///
    /// If the singleton is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    pub fn try_get_singleton_as<T>(&self, name: impl AsArg<StringName>) -> Option<Gd<T>>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(name);
        self.get_singleton(name).and_then(|obj| obj.try_cast::<T>().ok())
    }

    /// Alias for [`get_singleton_as()`][Self::get_singleton_as].
    pub fn get_singleton_typed<T>(&self, name: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        self.get_singleton_as::<T>(name)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `PackedScene` class.
impl PackedScene {
    /// ⚠️ Instantiates the scene as type `T`, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the scene is not type `T` or inherited.
    pub fn instantiate_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_instantiate_as::<T>()
            .unwrap_or_else(|| panic!("Failed to instantiate {to}", to = T::class_id()))
    }

    /// Instantiates the scene as type `T` (fallible).
    ///
    /// If the scene is not type `T` or inherited.
    pub fn try_instantiate_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.instantiate().and_then(|gd| gd.try_cast::<T>().ok())
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `WorkerThreadPool` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::WorkerThreadPool {
    /// Adds a Rust task to the thread pool.
    ///
    /// The returned task ID can be used with `wait_for_task_completion()` or `is_task_completed()`.
    #[must_use]
    pub fn add_rust_task<F>(&self, task: F) -> i64
    where
        F: FnOnce() + Send + 'static,
    {
        use crate::obj::Singleton;
        let mut gd = crate::classes::WorkerThreadPool::singleton();

        let callable = crate::builtin::Callable::from_once_fn("rust_task", move |_| {
            task();
        });
        gd.add_task(&callable)
    }

    /// Adds a Rust group task to the thread pool.
    ///
    /// The returned task ID can be used with `wait_for_group_task_completion()` or `is_group_task_completed()`.
    #[must_use]
    pub fn add_rust_group_task<F>(&self, task: F, elements: i32) -> i64
    where
        F: Fn(u32) + Send + Sync + 'static,
    {
        use crate::obj::Singleton;
        let mut gd = crate::classes::WorkerThreadPool::singleton();

        #[cfg(feature = "experimental-threads")]
        let callable = crate::builtin::Callable::from_sync_fn("rust_group_task", move |args| {
            let index = args[0].to::<u32>();
            task(index);
        });

        #[cfg(not(feature = "experimental-threads"))]
        let callable = crate::builtin::Callable::from_fn("rust_group_task", move |args| {
            let index = args[0].to::<u32>();
            task(index);
        });

        gd.add_group_task(&callable, elements)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `ResourceLoader` class.
impl ResourceLoader {
    /// ⚠️ Loads a resource from the filesystem located at `path`, panicking on error.
    ///
    /// # Panics
    /// If the resource cannot be loaded, or is not of type `T` or inherited.
    pub fn load_as<T>(&self, path: impl AsArg<GString>) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        crate::tools::load(path)
    }

    /// Loads a resource from the filesystem located at `path` (fallible).
    pub fn try_load_as<T>(&self, path: impl AsArg<GString>) -> Result<Gd<T>, crate::meta::error::IoError>
    where
        T: Inherits<Resource>,
    {
        crate::tools::try_load(path)
    }

    /// Alias for [`load_as()`][Self::load_as].
    pub fn load_typed<T>(&self, path: impl AsArg<GString>) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        self.load_as::<T>(path)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `Resource` class.
impl Resource {
    /// ⚠️ Duplicates the resource, panicking if the duplicate is not of type `T` or inherited.
    pub fn duplicate_as<T>(&self, subresources: bool) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        self.duplicate_ex()
            .deep(subresources)
            .done()
            .expect("Resource::duplicate() failed")
            .cast::<T>()
    }

    /// Alias for [`duplicate_as()`][Self::duplicate_as].
    pub fn duplicate_typed<T>(&self, subresources: bool) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        self.duplicate_as::<T>(subresources)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

impl ResourceSaver {
    /// ⚠️ Saves a resource to the filesystem at `path`, panicking on error.
    ///
    /// # Panics
    /// If the resource cannot be saved.
    pub fn save_as<T>(&self, obj: &Gd<T>, path: impl AsArg<GString>)
    where
        T: Inherits<Resource>,
    {
        crate::tools::save(obj, path)
    }

    /// Saves a resource to the filesystem at `path` (fallible).
    pub fn try_save_as<T>(&self, obj: &Gd<T>, path: impl AsArg<GString>) -> Result<(), crate::meta::error::IoError>
    where
        T: Inherits<Resource>,
    {
        crate::tools::try_save(obj, path)
    }
}
