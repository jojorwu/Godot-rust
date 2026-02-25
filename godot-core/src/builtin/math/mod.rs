/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod approx_eq;
mod float;
mod glam_helpers;
mod xform;

pub use approx_eq::ApproxEq;
pub use float::FloatExt;

/// Wraps `value` between `min` and `max`.
///
/// _Godot equivalent: @GlobalScope.wrapi()_
pub fn wrapi(value: i64, min: i64, max: i64) -> i64 {
    let range = max - min;
    if range == 0 {
        return min;
    }
    let mut result = (value - min) % range;
    if result < 0 {
        result += range;
    }
    result + min
}

// Internal glam re-exports
pub(crate) use glam_helpers::*;
pub use xform::XformInv;

pub use crate::{assert_eq_approx, assert_ne_approx};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equal_approx() {
        assert_eq_approx!(1.0, 1.000001);
        assert_ne_approx!(1.0, 2.0);
        assert_eq_approx!(1.0, 1.000001, "Message {}", "formatted");
        assert_ne_approx!(1.0, 2.0, "Message {}", "formatted");
    }
}
