/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::{Node, PackedScene};
use crate::obj::{Gd, Inherits};

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
        self.try_instantiate_as::<T>().unwrap_or_else(|| {
            panic!(
                "PackedScene::instantiate_as() for scene '{}' ({}) failed: root node is not of type {} (requested {})",
                self.get_path(),
                self.get_class(),
                T::class_id(),
                std::any::type_name::<T>()
            )
        })
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

    /// Alias for [`instantiate_as()`][Self::instantiate_as].
    #[inline]
    pub fn instantiate_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.instantiate_as::<T>()
    }
}
