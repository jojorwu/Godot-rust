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
    #[track_caller]
    pub fn instantiate_as<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.try_instantiate_as::<T>().unwrap_or_else(|| {
            let actual = self
                .instantiate()
                .map(|gd| format!("{:?}", gd.get_class()))
                .unwrap_or_else(|| "null".to_string());

            panic!(
                "{}::instantiate_as(): failed to instantiate scene as {to} ({rust_to}); actual type was {actual}.",
                std::any::type_name::<Self>(),
                to = T::class_id(),
                rust_to = std::any::type_name::<T>(),
            )
        })
    }

    /// Alias for [`instantiate_as()`][Self::instantiate_as].
    pub fn instantiate_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<Node>,
    {
        self.instantiate_as::<T>()
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
