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

use crate::builtin::{NodePath, StringName};
use crate::classes::{Node, PackedScene};
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
    pub fn add_rust_task<F>(&self, task: F) -> i64
    where
        F: FnOnce() + Send + 'static,
    {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::WorkerThreadPool>();
        let callable = crate::builtin::Callable::from_once_fn("rust_task", move |_| {
            task();
        });
        gd.add_task(&callable)
    }

    /// Adds a Rust group task to the thread pool.
    pub fn add_rust_group_task<F>(
        &self,
        task: F,
        elements: i32,
    ) -> i64
    where
        F: Fn(u32) + Send + Sync + 'static,
    {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::WorkerThreadPool>();

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
