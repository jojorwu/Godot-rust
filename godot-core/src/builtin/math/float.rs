/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::ApproxEq;

mod private {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Trait that provides Godot math functions as extensions on `f32` and `f64`.
pub trait FloatExt: private::Sealed + Copy {
    const CMP_EPSILON: Self;

    /// Linearly interpolates from `self` to `to` by `weight`.
    ///
    /// `weight` should be in the range `0.0 ..= 1.0`, but values outside this are allowed and will perform
    /// linear extrapolation.
    fn lerp(self, to: Self, weight: Self) -> Self;

    /// Check if two angles are approximately equal, by comparing the distance
    /// between the points on the unit circle with 0 using [`real::approx_eq`].
    fn is_angle_equal_approx(self, other: Self) -> bool;

    /// Check if `self` is within [`Self::CMP_EPSILON`] of `0.0`.
    fn is_zero_approx(self) -> bool;

    /// Returns the floating-point modulus of `self` divided by `pmod`, wrapping equally in positive and negative.
    fn fposmod(self, pmod: Self) -> Self;

    /// Returns the multiple of `step` that is closest to `self`.
    fn snapped(self, step: Self) -> Self;

    /// Godot's `sign` function, returns `0.0` when self is `0.0`.
    ///
    /// See also [`f32::signum`] and [`f64::signum`], which always return `-1.0` or `1.0` (or `NaN`).
    fn sign(self) -> Self;

    /// Returns the derivative at the given `t` on a one-dimensional Bézier curve defined by the given
    /// `control_1`, `control_2`, and `end` points.
    fn bezier_derivative(self, control_1: Self, control_2: Self, end: Self, t: Self) -> Self;

    /// Returns the point at the given `t` on a one-dimensional Bézier curve defined by the given
    /// `control_1`, `control_2`, and `end` points.
    fn bezier_interpolate(self, control_1: Self, control_2: Self, end: Self, t: Self) -> Self;

    /// Cubic interpolates between two values by the factor defined in `weight` with `pre` and `post` values.
    fn cubic_interpolate(self, to: Self, pre: Self, post: Self, weight: Self) -> Self;

    /// Cubic interpolates between two angles (in radians) by the factor defined in `weight` with `pre` and `post` values.
    fn cubic_interpolate_angle(self, to: Self, pre: Self, post: Self, weight: Self) -> Self;

    /// Cubic interpolates between two values by the factor defined in `weight` with `pre` and `post` values.
    /// It can perform smoother interpolation than [`cubic_interpolate`](FloatExt::cubic_interpolate) by the time values.
    #[allow(clippy::too_many_arguments)]
    fn cubic_interpolate_in_time(
        self,
        to: Self,
        pre: Self,
        post: Self,
        weight: Self,
        to_t: Self,
        pre_t: Self,
        post_t: Self,
    ) -> Self;

    /// Cubic interpolates between two angles (in radians) by the factor defined in `weight` with `pre` and `post` values.
    /// It can perform smoother interpolation than [`cubic_interpolate_angle`](FloatExt::cubic_interpolate_angle) by the time values.
    #[allow(clippy::too_many_arguments)]
    fn cubic_interpolate_angle_in_time(
        self,
        to: Self,
        pre: Self,
        post: Self,
        weight: Self,
        to_t: Self,
        pre_t: Self,
        post_t: Self,
    ) -> Self;

    /// Linearly interpolates between two angles (in radians) by a `weight` value
    /// between 0.0 and 1.0.
    ///
    /// Similar to [`lerp`][Self::lerp], but interpolates correctly when the angles wrap around
    /// [`TAU`][crate::builtin::real_consts::TAU].
    ///
    /// The resulting angle is not normalized.
    ///
    /// Note: This function lerps through the shortest path between `from` and
    /// `to`. However, when these two angles are approximately `PI + k * TAU` apart
    /// for any integer `k`, it's not obvious which way they lerp due to
    /// floating-point precision errors. For example, with single-precision floats
    /// `lerp_angle(0.0, PI, weight)` lerps clockwise, while `lerp_angle(0.0, PI + 3.0 * TAU, weight)`
    /// lerps counter-clockwise.
    ///
    /// _Godot equivalent: @GlobalScope.lerp_angle()_
    fn lerp_angle(self, to: Self, weight: Self) -> Self;

    /// Returns the shortest difference between two angles (in radians).
    ///
    /// _Godot equivalent: @GlobalScope.angle_difference()_
    fn angle_difference(self, to: Self) -> Self;

    /// Rotates `self` toward `to` by the fixed `delta` amount. Will not go past the final value.
    ///
    /// _Godot equivalent: @GlobalScope.rotate_toward()_
    fn rotate_toward(self, to: Self, delta: Self) -> Self;

    /// Returns the result of the inverse linear interpolation between `self` and `to` by the given `value`.
    ///
    /// _Godot equivalent: @GlobalScope.inverse_lerp()_
    fn inverse_lerp(self, to: Self, value: Self) -> Self;

    /// Returns the linear value from a decibel value.
    ///
    /// _Godot equivalent: @GlobalScope.db_to_linear()_
    fn db_to_linear(self) -> Self;

    /// Returns the decibel value from a linear value.
    ///
    /// _Godot equivalent: @GlobalScope.linear_to_db()_
    fn linear_to_db(self) -> Self;

    /// Returns the multiple of `step` that is closest to `self`.
    ///
    /// _Godot equivalent: @GlobalScope.stepify()_
    fn stepify(self, step: Self) -> Self;

    /// Maps a `value` from range `[istart, istop]` to `[ostart, ostop]`.
    ///
    /// _Godot equivalent: @GlobalScope.remap()_
    fn remap(self, istart: Self, istop: Self, ostart: Self, ostop: Self) -> Self;

    /// Returns a value smoothed between `self` and `to` based on `x`.
    ///
    /// _Godot equivalent: @GlobalScope.smoothstep()_
    fn smoothstep(self, to: Self, x: Self) -> Self;

    /// Returns a new value moved toward `to` by the fixed `delta` amount. Will not go past the final value.
    ///
    /// _Godot equivalent: @GlobalScope.move_toward()_
    fn move_toward(self, to: Self, delta: Self) -> Self;

    /// Returns a value that oscillates between `0.0` and `length`.
    ///
    /// _Godot equivalent: @GlobalScope.pingpong()_
    fn pingpong(self, length: Self) -> Self;

    /// Returns an eased value based on the `curve` value.
    ///
    /// _Godot equivalent: @GlobalScope.ease()_
    fn ease(self, curve: Self) -> Self;

    /// Returns the sine of `self` divided by `self`.
    ///
    /// _Godot equivalent: @GlobalScope.sinc()_
    fn sinc(self) -> Self;

    /// Returns the sine of `PI * self` divided by `PI * self`.
    ///
    /// _Godot equivalent: @GlobalScope.sincn()_
    fn sincn(self) -> Self;

    /// Returns the arc sine of `self`, clamped between -1 and 1 to avoid NaN.
    ///
    /// _Godot equivalent: @GlobalScope.asin()_
    fn asin_clamped(self) -> Self;

    /// Returns the arc cosine of `self`, clamped between -1 and 1 to avoid NaN.
    ///
    /// _Godot equivalent: @GlobalScope.acos()_
    fn acos_clamped(self) -> Self;

    /// Returns the inverse hyperbolic cosine of `self`, clamped to 1 and above to avoid NaN.
    ///
    /// _Godot equivalent: @GlobalScope.acosh()_
    fn acosh_clamped(self) -> Self;

    /// Returns the inverse hyperbolic tangent of `self`.
    ///
    /// _Godot equivalent: @GlobalScope.atanh()_
    fn atanh_clamped(self) -> Self;

    /// Wraps `self` between `min` and `max`.
    ///
    /// _Godot equivalent: @GlobalScope.wrapf()_
    fn wrap(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_float_ext {
    ($Ty:ty, $consts:path, $to_real:ident) => {
        impl FloatExt for $Ty {
            const CMP_EPSILON: Self = 0.00001;

            fn lerp(self, to: Self, t: Self) -> Self {
                self + ((to - self) * t)
            }

            fn is_angle_equal_approx(self, other: Self) -> bool {
                use $consts;

                let difference = (other - self) % consts::TAU;
                let distance = (2.0 * difference) % consts::TAU - difference;
                distance.is_zero_approx()
            }

            fn is_zero_approx(self) -> bool {
                self.abs() < Self::CMP_EPSILON
            }

            fn fposmod(self, pmod: Self) -> Self {
                let mut value = self % pmod;
                if (value < 0.0 && pmod > 0.0) || (value > 0.0 && pmod < 0.0) {
                    value += pmod;
                }
                value
            }

            fn snapped(mut self, step: Self) -> Self {
                if step != 0.0 {
                    self = (self / step + 0.5).floor() * step
                }
                self
            }

            fn sign(self) -> Self {
                use std::cmp::Ordering;

                match self.partial_cmp(&0.0) {
                    Some(Ordering::Equal) => 0.0,
                    Some(Ordering::Greater) => 1.0,
                    Some(Ordering::Less) => -1.0,
                    // `self` is `NaN`
                    None => Self::NAN,
                }
            }

            fn bezier_derivative(
                self,
                control_1: Self,
                control_2: Self,
                end: Self,
                t: Self,
            ) -> Self {
                let omt = 1.0 - t;
                let omt2 = omt * omt;
                let t2 = t * t;
                (control_1 - self) * 3.0 * omt2
                    + (control_2 - control_1) * 6.0 * omt * t
                    + (end - control_2) * 3.0 * t2
            }

            fn bezier_interpolate(
                self,
                control_1: Self,
                control_2: Self,
                end: Self,
                t: Self,
            ) -> Self {
                let omt = 1.0 - t;
                let omt2 = omt * omt;
                let omt3 = omt2 * omt;
                let t2 = t * t;
                let t3 = t2 * t;
                self * omt3 + control_1 * omt2 * t * 3.0 + control_2 * omt * t2 * 3.0 + end * t3
            }

            fn cubic_interpolate(self, to: Self, pre: Self, post: Self, weight: Self) -> Self {
                0.5 * ((self * 2.0)
                    + (-pre + to) * weight
                    + (2.0 * pre - 5.0 * self + 4.0 * to - post) * (weight * weight)
                    + (-pre + 3.0 * self - 3.0 * to + post) * (weight * weight * weight))
            }

            fn cubic_interpolate_angle(self, to: Self, pre: Self, post: Self, weight: Self) -> Self {
                use $consts;
                let from_rot = self % consts::TAU;

                let pre_diff = (pre - from_rot) % consts::TAU;
                let pre_rot = from_rot + (2.0 * pre_diff) % consts::TAU - pre_diff;

                let to_diff = (to - from_rot) % consts::TAU;
                let to_rot = from_rot + (2.0 * to_diff) % consts::TAU - to_diff;

                let post_diff = (post - to_rot) % consts::TAU;
                let post_rot = to_rot + (2.0 * post_diff) % consts::TAU - post_diff;

                from_rot.cubic_interpolate(to_rot, pre_rot, post_rot, weight)
            }

            fn cubic_interpolate_in_time(
                self,
                to: Self,
                pre: Self,
                post: Self,
                weight: Self,
                to_t: Self,
                pre_t: Self,
                post_t: Self,
            ) -> Self {
                let t = Self::lerp(0.0, to_t, weight);

                let a1 = Self::lerp(
                    pre,
                    self,
                    if pre_t == 0.0 {
                        0.0
                    } else {
                        (t - pre_t) / -pre_t
                    },
                );

                let a2 = Self::lerp(self, to, if to_t == 0.0 { 0.5 } else { t / to_t });

                let a3 = Self::lerp(
                    to,
                    post,
                    if post_t - to_t == 0.0 {
                        1.0
                    } else {
                        (t - to_t) / (post_t - to_t)
                    },
                );

                let b1 = Self::lerp(
                    a1,
                    a2,
                    if to_t - pre_t == 0.0 {
                        0.0
                    } else {
                        (t - pre_t) / (to_t - pre_t)
                    },
                );

                let b2 = Self::lerp(a2, a3, if post_t == 0.0 { 1.0 } else { t / post_t });

                Self::lerp(b1, b2, if to_t == 0.0 { 0.5 } else { t / to_t })
            }

            fn cubic_interpolate_angle_in_time(
                self,
                to: Self,
                pre: Self,
                post: Self,
                weight: Self,
                to_t: Self,
                pre_t: Self,
                post_t: Self,
            ) -> Self {
                use $consts;
                let from_rot = self % consts::TAU;

                let pre_diff = (pre - from_rot) % consts::TAU;
                let pre_rot = from_rot + (2.0 * pre_diff) % consts::TAU - pre_diff;

                let to_diff = (to - from_rot) % consts::TAU;
                let to_rot = from_rot + (2.0 * to_diff) % consts::TAU - to_diff;

                let post_diff = (post - to_rot) % consts::TAU;
                let post_rot = to_rot + (2.0 * post_diff) % consts::TAU - post_diff;

                from_rot.cubic_interpolate_in_time(
                    to_rot, pre_rot, post_rot, weight, to_t, pre_t, post_t,
                )
            }

            fn lerp_angle(self, to: Self, weight: Self) -> Self {
                self + self.angle_difference(to) * weight
            }

            fn angle_difference(self, to: Self) -> Self {
                use $consts;

                // Rust's % operator (remainder) matches Godot's C++ fmod() behavior (sign matches dividend).
                // This ensures consistent angle interpolation between Rust and Godot.
                let difference = (to - self) % consts::TAU;
                (2.0 * difference) % consts::TAU - difference
            }

            fn rotate_toward(self, to: Self, delta: Self) -> Self {
                self + self.angle_difference(to).clamp(-delta, delta)
            }

            fn inverse_lerp(self, to: Self, value: Self) -> Self {
                if self == to {
                    0.0
                } else {
                    (value - self) / (to - self)
                }
            }

            fn db_to_linear(self) -> Self {
                (self * 0.115_129_254_649_702_3).exp()
            }

            fn linear_to_db(self) -> Self {
                self.ln() * 8.685_889_638_065_037
            }

            fn stepify(self, step: Self) -> Self {
                self.snapped(step)
            }

            fn remap(self, istart: Self, istop: Self, ostart: Self, ostop: Self) -> Self {
                ostart.lerp(ostop, istart.inverse_lerp(istop, self))
            }

            fn smoothstep(self, to: Self, x: Self) -> Self {
                if self == to {
                    0.0
                } else {
                    let t = ((x - self) / (to - self)).clamp(0.0, 1.0);
                    t * t * (3.0 - 2.0 * t)
                }
            }

            fn move_toward(self, to: Self, delta: Self) -> Self {
                if (to - self).abs() <= delta {
                    to
                } else {
                    self + (to - self).signum() * delta
                }
            }

            fn pingpong(self, length: Self) -> Self {
                if length != 0.0 {
                    ((self - length).fposmod(length * 2.0) - length).abs()
                } else {
                    0.0
                }
            }

            fn ease(self, c: Self) -> Self {
                let x = self.clamp(0.0, 1.0);
                if c > 0.0 {
                    if c < 1.0 {
                        1.0 - (1.0 - x).powf(1.0 / c)
                    } else {
                        x.powf(c)
                    }
                } else if c < 0.0 {
                    if x < 0.5 {
                        (x * 2.0).powf(-c) * 0.5
                    } else {
                        (1.0 - ((1.0 - x) * 2.0).powf(-c)) * 0.5 + 0.5
                    }
                } else {
                    0.0
                }
            }

            fn sinc(self) -> Self {
                if self == 0.0 {
                    1.0
                } else {
                    self.sin() / self
                }
            }

            fn sincn(self) -> Self {
                use $consts;
                let pi = consts::PI;
                (self * pi).sinc()
            }

            fn asin_clamped(self) -> Self {
                use $consts;
                let pi = consts::PI;
                if self < -1.0 {
                    -pi / 2.0
                } else if self > 1.0 {
                    pi / 2.0
                } else {
                    self.asin()
                }
            }

            fn acos_clamped(self) -> Self {
                use $consts;
                let pi = consts::PI;
                if self < -1.0 {
                    pi
                } else if self > 1.0 {
                    0.0
                } else {
                    self.acos()
                }
            }

            fn acosh_clamped(self) -> Self {
                if self < 1.0 {
                    0.0
                } else {
                    self.acosh()
                }
            }

            fn atanh_clamped(self) -> Self {
                if self <= -1.0 {
                    Self::NEG_INFINITY
                } else if self >= 1.0 {
                    Self::INFINITY
                } else {
                    self.atanh()
                }
            }

            fn wrap(self, min: Self, max: Self) -> Self {
                let range = max - min;
                if range.is_zero_approx() {
                    return min;
                }
                let mut result = self - (range * ((self - min) / range).floor());
                if result.approx_eq(&max) {
                    result = min;
                }
                result
            }
        }

        impl ApproxEq for $Ty {
            #[inline]
            fn approx_eq(&self, other: &Self) -> bool {
                if self == other {
                    return true;
                }
                let max = self.abs().max(other.abs()).max(1.0);
                (self - other).abs() < Self::CMP_EPSILON * max
            }
        }
    };
}

impl_float_ext!(f32, std::f32::consts, from_f32);
impl_float_ext!(f64, std::f64::consts, from_f64);

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_approx;

    // Create functions that take references for use in `assert_eq/ne_approx`.
    fn is_angle_equal_approx_f32(a: &f32, b: &f32) -> bool {
        a.is_angle_equal_approx(*b)
    }

    fn is_angle_equal_approx_f64(a: &f64, b: &f64) -> bool {
        a.is_angle_equal_approx(*b)
    }

    #[test]
    fn angle_equal_approx_f32() {
        use std::f32::consts::{PI, TAU};

        assert_eq_approx!(1.0, 1.000001, fn = is_angle_equal_approx_f32);
        assert_eq_approx!(0.0, TAU, fn = is_angle_equal_approx_f32);
        assert_eq_approx!(PI, -PI, fn = is_angle_equal_approx_f32);
        assert_eq_approx!(4.45783, -(TAU - 4.45783), fn = is_angle_equal_approx_f32);
        assert_eq_approx!(31.0 * PI, -13.0 * PI, fn = is_angle_equal_approx_f32);
    }

    #[test]
    fn angle_equal_approx_f64() {
        use std::f64::consts::{PI, TAU};

        assert_eq_approx!(1.0, 1.000001, fn = is_angle_equal_approx_f64);
        assert_eq_approx!(0.0, TAU, fn = is_angle_equal_approx_f64);
        assert_eq_approx!(PI, -PI, fn = is_angle_equal_approx_f64);
        assert_eq_approx!(4.45783, -(TAU - 4.45783), fn = is_angle_equal_approx_f64);
        assert_eq_approx!(31.0 * PI, -13.0 * PI, fn = is_angle_equal_approx_f64);
    }

    #[test]
    #[should_panic(expected = "I am inside format")]
    fn eq_approx_fail_with_message() {
        assert_eq_approx!(1.0, 2.0, "I am inside {}", "format");
    }

    // As mentioned in the docs for `lerp_angle`, direction can be unpredictable
    // when lerping towards PI radians, this also means it's different for single vs
    // double precision floats.

    #[test]
    fn lerp_angle_test_f32() {
        use std::f32::consts::{FRAC_PI_2, PI, TAU};

        assert_eq_approx!(f32::lerp_angle(0.0, PI, 0.5), -FRAC_PI_2, fn = is_angle_equal_approx_f32);

        assert_eq_approx!(
            f32::lerp_angle(0.0, PI + 3.0 * TAU, 0.5),
            FRAC_PI_2,
            fn = is_angle_equal_approx_f32
        );

        let angle = PI * 2.0 / 3.0;
        assert_eq_approx!(
            f32::lerp_angle(-5.0 * TAU, angle + 3.0 * TAU, 0.5),
            (angle / 2.0),
            fn = is_angle_equal_approx_f32
        );
    }

    #[test]
    fn lerp_angle_test_f64() {
        use std::f64::consts::{FRAC_PI_2, PI, TAU};

        assert_eq_approx!(f64::lerp_angle(0.0, PI, 0.5), -FRAC_PI_2, fn = is_angle_equal_approx_f64);

        assert_eq_approx!(
            f64::lerp_angle(0.0, PI + 3.0 * TAU, 0.5),
            -FRAC_PI_2,
            fn = is_angle_equal_approx_f64
        );

        let angle = PI * 2.0 / 3.0;
        assert_eq_approx!(
            f64::lerp_angle(-5.0 * TAU, angle + 3.0 * TAU, 0.5),
            (angle / 2.0),
            fn = is_angle_equal_approx_f64
        );
    }

    #[test]
    fn inverse_lerp() {
        assert_eq_approx!(f32::inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq_approx!(f32::inverse_lerp(10.0, 0.0, 5.0), 0.5);
        assert_eq_approx!(f64::inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq_approx!(f32::inverse_lerp(1.0, 1.0, 1.0), 0.0);
    }

    #[test]
    fn remap() {
        assert_eq_approx!(f32::remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq_approx!(f64::remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq_approx!(f32::remap(1.0, 1.0, 1.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn smoothstep() {
        assert_eq_approx!(f32::smoothstep(0.0, 2.0, -1.0), 0.0);
        assert_eq_approx!(f32::smoothstep(0.0, 2.0, 1.0), 0.5);
        assert_eq_approx!(f32::smoothstep(0.0, 2.0, 3.0), 1.0);
        assert_eq_approx!(f32::smoothstep(1.0, 1.0, 0.5), 0.0);
        assert_eq_approx!(f32::smoothstep(1.0, 1.0, 1.5), 0.0);
    }

    #[test]
    fn move_toward() {
        assert_eq_approx!(f32::move_toward(0.0, 10.0, 4.0), 4.0);
        assert_eq_approx!(f32::move_toward(10.0, 0.0, 4.0), 6.0);
        assert_eq_approx!(f32::move_toward(5.0, 10.0, 10.0), 10.0);
    }

    #[test]
    fn pingpong() {
        assert_eq_approx!(f32::pingpong(1.5, 1.0), 0.5);
        assert_eq_approx!(f32::pingpong(2.5, 1.0), 0.5);
        assert_eq_approx!(f32::pingpong(-0.5, 1.0), 0.5);
    }

    #[test]
    fn angle_difference() {
        use std::f32::consts::PI;
        assert_eq_approx!(f32::angle_difference(0.0, PI / 2.0), PI / 2.0);
        assert_eq_approx!(f32::angle_difference(0.0, -PI / 2.0), -PI / 2.0);
        assert_eq_approx!(f32::angle_difference(PI, -PI / 2.0), PI / 2.0);
    }

    #[test]
    fn rotate_toward() {
        use std::f32::consts::PI;
        assert_eq_approx!(f32::rotate_toward(0.0, PI / 2.0, PI / 4.0), PI / 4.0);
        assert_eq_approx!(f32::rotate_toward(PI / 2.0, 0.0, PI / 4.0), PI / 4.0);
        assert_eq_approx!(f32::rotate_toward(0.0, PI, PI / 2.0).abs(), PI / 2.0);
    }

    #[test]
    fn db_linear() {
        assert_eq_approx!(f32::db_to_linear(0.0), 1.0);
        assert_eq_approx!(f32::linear_to_db(1.0), 0.0);
        assert_eq_approx!(f32::db_to_linear(6.0206), 2.0);
        assert_eq_approx!(f32::linear_to_db(2.0), 6.0206);
    }

    #[test]
    fn ease() {
        assert_eq_approx!(f32::ease(0.5, 1.0), 0.5);
        assert_eq_approx!(f32::ease(0.5, 2.0), 0.25);
        assert_eq_approx!(f32::ease(0.5, -2.0), 0.5);
        assert_eq_approx!(f32::ease(0.25, -2.0), 0.125);
        assert_eq_approx!(f32::ease(0.75, -2.0), 0.875);
    }
}

#[cfg(test)]
mod test_expansions {
    use super::*;
    use crate::assert_eq_approx;

    #[test]
    fn cubic_interpolate_angle() {
        use std::f32::consts::PI;
        let a = f32::cubic_interpolate_angle(0.1, PI - 0.1, 0.0, PI, 0.5);
        assert_eq_approx!(a, PI / 2.0);
    }

    #[test]
    fn stepify() {
        assert_eq_approx!(3.14159f32.stepify(0.01), 3.14);
        assert_eq_approx!(12.345f64.stepify(0.1), 12.3);
    }
}

#[cfg(test)]
mod test_clamped_trig {
    use super::*;
    use crate::assert_eq_approx;

    #[test]
    fn clamped_trig() {
        use std::f32::consts::PI;
        assert_eq_approx!(2.0f32.asin_clamped(), PI / 2.0);
        assert_eq_approx!((-2.0f32).asin_clamped(), -PI / 2.0);
        assert_eq_approx!(2.0f32.acos_clamped(), 0.0);
        assert_eq_approx!((-2.0f32).acos_clamped(), PI);
        assert_eq_approx!(0.5f32.acosh_clamped(), 0.0);
        assert_eq_approx!(1.5f32.acosh_clamped(), 1.5f32.acosh());
        assert_eq_approx!(1.0f32.atanh_clamped(), f32::INFINITY);
        assert_eq_approx!((-1.0f32).atanh_clamped(), f32::NEG_INFINITY);
    }

    #[test]
    fn sinc_test() {
        assert_eq_approx!(0.0f32.sinc(), 1.0);
        assert_eq_approx!(1.0f32.sinc(), 1.0f32.sin());
        assert_eq_approx!(0.0f32.sincn(), 1.0);
        assert_eq_approx!(0.5f32.sincn(), (std::f32::consts::PI * 0.5).sin() / (std::f32::consts::PI * 0.5));
    }

    #[test]
    fn wrap_test() {
        assert_eq_approx!(1.5f32.wrap(0.0, 1.0), 0.5);
        assert_eq_approx!(0.5f32.wrap(0.0, 1.0), 0.5);
        assert_eq_approx!((-0.5f32).wrap(0.0, 1.0), 0.5);
        assert_eq_approx!(1.0f32.wrap(0.0, 1.0), 0.0);
    }
}
