/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Server-related functionality and RAII wrappers.

pub mod rendering;

#[cfg(feature = "codegen-full")]
pub mod display;

#[cfg(all(feature = "codegen-full", feature = "experimental-godot-api"))]
pub mod navigation;

#[cfg(feature = "codegen-full")]
pub mod physics;

#[cfg(feature = "codegen-full")]
pub mod rendering_device;

#[cfg(feature = "codegen-full")]
pub mod text;
