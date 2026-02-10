/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(feature = "codegen-full")]
use crate::builtin::{GString, StringName};
#[cfg(feature = "codegen-full")]
use crate::classes::Object;
#[cfg(feature = "codegen-full")]
use crate::meta::{arg_into_ref, AsArg, FromGodot};
#[cfg(feature = "codegen-full")]
use crate::obj::{Gd, Inherits};

/// Manual extensions for the `ClassDb` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::ClassDb {
    /// ⚠️ Instantiates a class by name, panicking if it cannot be created or bad type.
    pub fn instantiate_as<T>(&self, class: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(class);
        let variant = self.instantiate(class);
        if variant.is_nil() {
            panic!("ClassDB::instantiate_as(): failed to instantiate class '{class}' (returned Nil)");
        }
        let obj = variant.try_to::<Gd<Object>>().unwrap_or_else(|err| {
            panic!("ClassDB::instantiate_as(): failed to convert instance of '{class}' to Gd<Object>: {err}");
        });

        obj.try_cast::<T>().unwrap_or_else(|obj| {
            panic!(
                "ClassDB::instantiate_as(): class '{class}' (instance {obj:?}) is not of type {to}",
                to = T::class_id()
            );
        })
    }

    /// Instantiates a class by name (fallible).
    #[inline]
    pub fn try_instantiate_as<T>(&self, class: impl AsArg<StringName>) -> Option<Gd<T>>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(class);
        self.instantiate(class)
            .try_to::<Gd<Object>>()
            .ok()?
            .try_cast::<T>()
            .ok()
    }

    /// Alias for [`instantiate_as()`][Self::instantiate_as].
    #[inline]
    pub fn instantiate_typed<T>(&self, class: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        self.instantiate_as::<T>(class)
    }
}

/// Manual extensions for the `EditorInterface` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::EditorInterface {
    /// Retrieves the editor main screen control, cast to type `T`.
    pub fn get_editor_main_screen_as<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_editor_main_screen()
            .expect("Editor main screen not found")
            .upcast::<crate::classes::Control>()
            .cast::<T>()
    }

    /// Alias for [`get_editor_main_screen_as()`][Self::get_editor_main_screen_as].
    pub fn get_editor_main_screen_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_editor_main_screen_as::<T>()
    }

    /// Retrieves the editor base control, cast to type `T`.
    pub fn get_base_control_as<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_base_control()
            .expect("Editor base control not found")
            .upcast::<crate::classes::Control>()
            .cast::<T>()
    }

    /// Alias for [`get_base_control_as()`][Self::get_base_control_as].
    pub fn get_base_control_typed<T>(&self) -> Gd<T>
    where
        T: Inherits<crate::classes::Control>,
    {
        self.get_base_control_as::<T>()
    }
}

/// Manual extensions for the `ProjectSettings` class.
#[cfg(feature = "codegen-full")]
impl crate::classes::ProjectSettings {
    /// ⚠️ Retrieves a setting value, panicking if not found or cannot be converted to `T`.
    ///
    /// # Panics
    /// If the setting is not found, or if its value cannot be converted to `T`.
    pub fn get_setting_as<T: FromGodot>(&self, name: impl AsArg<GString>) -> T {
        arg_into_ref!(name);
        let variant = self.get_setting(name);
        if variant.is_nil() {
            panic!("ProjectSettings::get_setting_as(): setting '{name}' not found (returned Nil)");
        }
        variant.try_to::<T>().unwrap_or_else(|err| {
            panic!("ProjectSettings::get_setting_as(): setting '{name}' conversion failed: {err}");
        })
    }

    /// Retrieves a setting value (fallible).
    ///
    /// If the setting is not found, or if its value cannot be converted to `T`,
    /// `None` will be returned.
    #[inline]
    pub fn try_get_setting_as<T: FromGodot>(&self, name: impl AsArg<GString>) -> Option<T> {
        self.get_setting(name).try_to::<T>().ok()
    }

    /// Alias for [`get_setting_as()`][Self::get_setting_as].
    #[inline]
    pub fn get_setting_typed<T: FromGodot>(&self, name: impl AsArg<GString>) -> T {
        self.get_setting_as::<T>(name)
    }
}

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
        arg_into_ref!(name);
        let obj = self.get_singleton(name).unwrap_or_else(|| {
            panic!("Engine::get_singleton_as(): singleton '{name}' not found");
        });

        obj.try_cast::<T>().unwrap_or_else(|obj| {
            panic!(
                "Engine::get_singleton_as(): singleton '{name}' (instance {obj:?}) is not of type {to}",
                to = T::class_id()
            );
        })
    }

    /// Retrieves a singleton instance by name (fallible).
    ///
    /// If the singleton is not found, or if it does not have type `T` or inherited,
    /// `None` will be returned.
    #[inline]
    pub fn try_get_singleton_as<T>(&self, name: impl AsArg<StringName>) -> Option<Gd<T>>
    where
        T: Inherits<crate::classes::Object>,
    {
        arg_into_ref!(name);
        self.get_singleton(name).and_then(|obj| obj.try_cast::<T>().ok())
    }

    /// Alias for [`get_singleton_as()`][Self::get_singleton_as].
    #[inline]
    pub fn get_singleton_typed<T>(&self, name: impl AsArg<StringName>) -> Gd<T>
    where
        T: Inherits<crate::classes::Object>,
    {
        self.get_singleton_as::<T>(name)
    }
}

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
