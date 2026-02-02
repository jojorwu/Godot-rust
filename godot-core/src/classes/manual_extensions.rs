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

use crate::builtin::{GString, NodePath, StringName};
use crate::classes::{Node, PackedScene, Resource, ResourceLoader, ResourceSaver};
use crate::meta::{arg_into_ref, AsArg};
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
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Manual extensions for the `ResourceSaver` class.
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
