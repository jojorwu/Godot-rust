/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::fmt;

/// Error that can occur while using Godot collections.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CollectionError {
    /// Index is out of bounds.
    OutOfBounds { index: usize, len: usize },

    /// Capacity overflow or allocation failure.
    Capacity,

    /// Encoding or decoding failure.
    Encoding,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { index, len } => {
                write!(f, "collection index {index} is out of bounds (length {len})")
            }
            Self::Capacity => write!(f, "collection capacity overflow or allocation failure"),
            Self::Encoding => write!(f, "collection encoding or decoding failure"),
        }
    }
}

impl Error for CollectionError {}
