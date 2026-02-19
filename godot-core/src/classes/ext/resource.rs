/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::GString;
use crate::classes::{Resource, ResourceLoader, ResourceSaver};
use crate::meta::AsArg;
use crate::obj::{Gd, Inherits};

/// Manual extensions for the `ResourceLoader` class.
impl ResourceLoader {
    /// ⚠️ Loads a resource from the filesystem located at `path`, panicking on error.
    ///
    /// # Panics
    /// If the resource cannot be loaded, or is not of type `T` or inherited.
    #[track_caller]
    pub fn load_as<T>(&self, path: impl AsArg<GString>) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        crate::tools::load(path)
    }

    /// Loads a resource from the filesystem located at `path` (fallible).
    pub fn try_load_as<T>(
        &self,
        path: impl AsArg<GString>,
    ) -> Result<Gd<T>, crate::meta::error::IoError>
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

/// Manual extensions for the `Resource` class.
impl Resource {
    /// ⚠️ Duplicates the resource, panicking if the duplicate is not of type `T` or inherited.
    #[track_caller]
    pub fn duplicate_as<T>(&self, subresources: bool) -> Gd<T>
    where
        T: Inherits<Resource>,
    {
        self.duplicate_ex()
            .deep(subresources)
            .done()
            .unwrap_or_else(|| panic!("{}::duplicate() failed", std::any::type_name::<Self>()))
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

impl ResourceSaver {
    /// ⚠️ Saves a resource to the filesystem at `path`, panicking on error.
    ///
    /// # Panics
    /// If the resource cannot be saved.
    #[track_caller]
    pub fn save_as<T>(&self, obj: &Gd<T>, path: impl AsArg<GString>)
    where
        T: Inherits<Resource>,
    {
        crate::tools::save(obj, path)
    }

    /// Saves a resource to the filesystem at `path` (fallible).
    pub fn try_save_as<T>(
        &self,
        obj: &Gd<T>,
        path: impl AsArg<GString>,
    ) -> Result<(), crate::meta::error::IoError>
    where
        T: Inherits<Resource>,
    {
        crate::tools::try_save(obj, path)
    }

    /// Alias for [`save_as()`][Self::save_as].
    pub fn save_typed<T>(&self, obj: &Gd<T>, path: impl AsArg<GString>)
    where
        T: Inherits<Resource>,
    {
        self.save_as(obj, path)
    }
}
