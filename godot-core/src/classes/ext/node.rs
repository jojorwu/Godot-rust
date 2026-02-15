/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{GString, NodePath, StringName};
use crate::classes::{Node, SceneTree};
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
    #[inline]
    #[track_caller]
    pub fn get_node_as<T>(&self, path: impl AsArg<NodePath>) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(path);

        match self.try_get_node_as::<T>(path) {
            Ok(node) => node,
            Err(err) => panic!(
                "{}::get_node_as(): {err} at path `{path}` (target type {to})",
                std::any::type_name::<Self>(),
                to = std::any::type_name::<T>(),
            ),
        }
    }

    /// Retrieves the node at path `path` (fallible).
    ///
    /// If the node is not found, or if it does not have type `T` or inherited,
    /// a [`GetNodeError`] will be returned.
    #[inline]
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
    #[inline]
    pub fn get_node_or_none<T>(&self, path: impl AsArg<NodePath>) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.try_get_node_as::<T>(path).ok()
    }

    /// Alias for [`get_node_as()`][Self::get_node_as].
    #[inline]
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
    #[inline]
    #[track_caller]
    pub fn get_parent_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_parent_as::<T>().unwrap_or_else(|| {
            panic!(
                "{}::get_parent_as(): parent not found or bad type (target type {to})",
                std::any::type_name::<Self>(),
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves the parent node (fallible).
    ///
    /// If the parent is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    #[inline]
    pub fn try_get_parent_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_parent()
            .and_then(|parent| parent.try_cast::<T>().ok())
    }

    /// Alias for [`get_parent_as()`][Self::get_parent_as].
    #[inline]
    pub fn get_parent_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_parent_as::<T>()
    }

    /// ⚠️ Retrieves the owner node, panicking if not found or bad type.
    ///
    /// # Panics
    /// If the owner is not found, or if it does not have type `T` or inherited.
    #[inline]
    #[track_caller]
    pub fn get_owner_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_owner_as::<T>().unwrap_or_else(|| {
            panic!(
                "{}::get_owner_as(): owner not found or bad type (target type {to})",
                std::any::type_name::<Self>(),
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves the owner node (fallible).
    ///
    /// If the owner is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    #[inline]
    pub fn try_get_owner_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_owner()
            .and_then(|owner| owner.try_cast::<T>().ok())
    }

    /// Alias for [`get_owner_as()`][Self::get_owner_as].
    #[inline]
    pub fn get_owner_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_owner_as::<T>()
    }

    /// ⚠️ Retrieves the child node at index `index`, panicking if out of bounds or bad type.
    ///
    /// # Panics
    /// If `index` is out of bounds, or if the node does not have type `T` or inherited.
    #[inline]
    #[track_caller]
    pub fn get_child_as<T>(&self, index: usize) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_child_as::<T>(index).unwrap_or_else(|| {
            panic!(
                "{}::get_child_as(): index {index} out of bounds or bad type (target type {to})",
                std::any::type_name::<Self>(),
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves the child node at index `index` (fallible).
    ///
    /// If `index` is out of bounds, or if the node does not have type `T` or inherited,
    /// `None` will be returned.
    #[inline]
    pub fn try_get_child_as<T>(&self, index: usize) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_child(index as i32)
            .and_then(|node| node.try_cast::<T>().ok())
    }

    /// Alias for [`get_child_as()`][Self::get_child_as].
    #[inline]
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

    /// Alias for [`get_children_as()`][Self::get_children_as].
    pub fn get_children_typed<T>(&self) -> Vec<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_children_as::<T>()
    }

    /// Finds the first child whose name matches `pattern`, cast to type `T`.
    ///
    /// If no child is found or if it cannot be cast to `T`, `None` is returned.
    #[inline]
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
    #[inline]
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

    /// Returns an iterator over children of type `T`.
    pub fn iter_children_typed<T>(&self) -> impl Iterator<Item = Gd<T>> + '_
    where
        T: Inherits<Node>,
    {
        self.get_children()
            .into_iter()
            .filter_map(|node| node.try_cast::<T>().ok())
    }

    /// Returns the first child of type `T`.
    pub fn get_first_child_typed<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.iter_children_typed::<T>().next()
    }

    /// ⚠️ Retrieves the scene tree, cast to type `T`, panicking if not found or bad type.
    #[inline]
    #[track_caller]
    pub fn get_tree_as<T>(&self) -> Gd<T>
    where
        T: Inherits<SceneTree>,
    {
        self.try_get_tree_as::<T>().unwrap_or_else(|| {
            panic!(
                "{}::get_tree_as(): scene tree not found or bad type (target type {to})",
                std::any::type_name::<Self>(),
                to = std::any::type_name::<T>()
            )
        })
    }

    /// Retrieves the scene tree, cast to type `T` (fallible).
    #[inline]
    pub fn try_get_tree_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<SceneTree>,
    {
        self.get_tree().and_then(|tree| tree.try_cast::<T>().ok())
    }

    /// Alias for [`get_tree_as()`][Self::get_tree_as].
    #[inline]
    pub fn get_tree_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<SceneTree>,
    {
        self.get_tree_as::<T>()
    }
}
