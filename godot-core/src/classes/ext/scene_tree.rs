/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::StringName;
use crate::classes::{Node, SceneTree};
use crate::meta::{arg_into_ref, AsArg};
use crate::obj::{Gd, Inherits};

/// Manual extensions for the `SceneTree` class.
impl SceneTree {
    /// ⚠️ Retrieves the first node in a group, cast to type `T`, panicking if not found or bad type.
    pub fn get_first_node_in_group_as<T>(&mut self, group: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(group);
        self.try_get_first_node_in_group_as::<T>(group).unwrap_or_else(|| {
            panic!(
                "SceneTree::get_first_node_in_group_as(): node in group '{group}' not found or bad type"
            )
        })
    }

    /// Retrieves the first node in a group, cast to type `T` (fallible).
    pub fn try_get_first_node_in_group_as<T>(&mut self, group: impl AsArg<StringName>) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(group);
        self.get_first_node_in_group(group)
            .and_then(|node| node.try_cast::<T>().ok())
    }

    /// Alias for [`get_first_node_in_group_as()`][Self::get_first_node_in_group_as].
    pub fn get_first_node_in_group_typed<T>(&mut self, group: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_first_node_in_group_as::<T>(group)
    }

    /// Retrieves all nodes in a group, cast to type `T`.
    ///
    /// Nodes that cannot be cast to `T` are ignored.
    pub fn get_nodes_in_group_as<T>(&mut self, group: impl AsArg<StringName>) -> Vec<Gd<T>>
    where
        T: Inherits<Node>,
    {
        arg_into_ref!(group);
        self.get_nodes_in_group(group)
            .iter_shared()
            .filter_map(|node| node.try_cast::<T>().ok())
            .collect()
    }

    /// ⚠️ Retrieves the current scene, cast to type `T`, panicking if not found or bad type.
    pub fn get_current_scene_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_get_current_scene_as::<T>()
            .expect("SceneTree::get_current_scene_as(): current scene not found or bad type")
    }

    /// Retrieves the current scene, cast to type `T` (fallible).
    pub fn try_get_current_scene_as<T>(&self) -> Option<Gd<T>>
    where
        T: Inherits<Node>,
    {
        self.get_current_scene()
            .and_then(|scene| scene.try_cast::<T>().ok())
    }

    /// Alias for [`get_current_scene_as()`][Self::get_current_scene_as].
    pub fn get_current_scene_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.get_current_scene_as::<T>()
    }
}
