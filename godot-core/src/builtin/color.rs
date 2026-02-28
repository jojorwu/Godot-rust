/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops;

use godot_ffi as sys;
use sys::{ffi_methods, ExtVariantType, GodotFfi};

use crate::builtin::color_hsv::rgba_to_hsva;
use crate::builtin::inner::InnerColor;
use crate::builtin::math::ApproxEq;
use crate::builtin::{ColorHsv, GString};
use crate::meta::{arg_into_ref, AsArg};

/// Color built-in type, in floating-point RGBA format.
///
/// Channel values are _typically_ in the range of 0 to 1, but this is not a requirement, and
/// values outside this range are explicitly allowed for e.g. High Dynamic Range (HDR).
///
/// To access its [**HSVA**](ColorHsv) representation, use [`Color::to_hsv`].
///
/// Predefined colors are available as constants, see the corresponding [`impl` block](#impl-Color-1).
///
/// # Godot docs
///
/// [`Color` (stable)](https://docs.godotengine.org/en/stable/classes/class_color.html)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// The color's red component.
    pub r: f32,

    /// The color's green component.
    pub g: f32,

    /// The color's blue component.
    pub b: f32,

    /// The color's alpha component. A value of 0 means that the color is fully transparent. A
    /// value of 1 means that the color is fully opaque.
    pub a: f32,
}

impl Color {
    /// Constructs a new `Color` with the given components.
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs a new `Color` with the given color components, and the alpha channel set to 1.
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Constructs a new `Color` with the given components as bytes. 0 is mapped to 0.0, 255 is
    /// mapped to 1.0.
    ///
    /// _Godot equivalent: the global `Color8` function_
    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_rgba(from_u8(r), from_u8(g), from_u8(b), from_u8(a))
    }

    /// Constructs a new `Color` with the given components as `u16` words.
    ///
    /// 0 is mapped to 0.0, 65535 (`0xFFFF`) is mapped to 1.0.
    pub const fn from_rgba16(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self::from_rgba(from_u16(r), from_u16(g), from_u16(b), from_u16(a))
    }

    /// Constructs a new `Color` from a 32-bits value with the given channel `order`.
    ///
    /// _Godot equivalent: `Color.hex`, if `ColorChannelOrder::Rgba` is used_
    pub const fn from_u32_rgba(u: u32, order: ColorChannelOrder) -> Self {
        let [r, g, b, a] = order.unpack(u.to_be_bytes());
        Color::from_rgba8(r, g, b, a)
    }

    /// Constructs a new `Color` from a 64-bits value with the given channel `order`.
    ///
    /// _Godot equivalent: `Color.hex64`, if `ColorChannelOrder::Rgba` is used_
    pub const fn from_u64_rgba(u: u64, order: ColorChannelOrder) -> Self {
        let [r, g, b, a] = order.unpack(to_be_words(u));
        Color::from_rgba16(r, g, b, a)
    }

    /// Constructs a `Color` from an HTML color code string. Valid values for the string are:
    ///
    /// - `#RRGGBBAA` and `RRGGBBAA` where each of `RR`, `GG`, `BB` and `AA` stands for two hex
    ///   digits (case insensitive).
    /// - `#RRGGBB` and `RRGGBB`. Equivalent to `#RRGGBBff`.
    /// - `#RGBA` and `RGBA` where each of `R`, `G`, `B` and `A` stands for a single hex digit.
    ///   Equivalent to `#RRGGBBAA`, i.e. each digit is repeated twice.
    /// - `#RGB` and `RGB`. Equivalent to `#RRGGBBff`.
    ///
    /// Returns `None` if the format is invalid.
    ///
    /// _Godot equivalent: `Color.from_html()`_
    pub fn from_html<S: AsArg<GString>>(html: S) -> Option<Self> {
        arg_into_ref!(html);
        Self::from_html_chars(html.chars())
    }

    /// Constructs a `Color` from an HTML color code string, using a `&str`.
    ///
    /// See [`Color::from_html()`] for more details.
    #[inline]
    pub fn from_html_str(html: &str) -> Option<Self> {
        let chars: Vec<char> = html.chars().collect();
        Self::from_html_chars(&chars)
    }

    fn from_html_chars(chars: &[char]) -> Option<Self> {
        if !Self::html_is_valid_chars(chars) {
            return None;
        }

        let start = if !chars.is_empty() && chars[0] == '#' {
            1
        } else {
            0
        };
        let hex = &chars[start..];
        let len = hex.len();

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let mut a = 1.0;

        let parse_hex = |c: char| c.to_digit(16);

        if len == 3 || len == 4 {
            r = parse_hex(hex[0])? as f32 / 15.0;
            g = parse_hex(hex[1])? as f32 / 15.0;
            b = parse_hex(hex[2])? as f32 / 15.0;
            if len == 4 {
                a = parse_hex(hex[3])? as f32 / 15.0;
            }
        } else if len == 6 || len == 8 {
            let parse_byte = |s: &[char]| -> Option<f32> {
                let hi = parse_hex(s[0])?;
                let lo = parse_hex(s[1])?;
                Some((hi * 16 + lo) as f32 / 255.0)
            };

            r = parse_byte(&hex[0..2])?;
            g = parse_byte(&hex[2..4])?;
            b = parse_byte(&hex[4..6])?;
            if len == 8 {
                a = parse_byte(&hex[6..8])?;
            }
        }

        Some(Color::from_rgba(r, g, b, a))
    }

    /// Returns `true` if `color` is a valid HTML color code string.
    ///
    /// _Godot equivalent: `Color.html_is_valid()`_
    pub fn html_is_valid<S: AsArg<GString>>(color: S) -> bool {
        arg_into_ref!(color);
        Self::html_is_valid_chars(color.chars())
    }

    /// Returns `true` if `html` is a valid HTML color code string, using a `&str`.
    ///
    /// See [`Color::html_is_valid()`] for more details.
    #[inline]
    pub fn html_is_valid_str(html: &str) -> bool {
        let chars: Vec<char> = html.chars().collect();
        Self::html_is_valid_chars(&chars)
    }

    fn html_is_valid_chars(chars: &[char]) -> bool {
        if chars.is_empty() {
            return false;
        }

        let start = if chars[0] == '#' { 1 } else { 0 };
        let hex = &chars[start..];
        let len = hex.len();

        if ![3, 4, 6, 8].contains(&len) {
            return false;
        }

        for &c in hex {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }

        true
    }

    /// Constructs a `Color` from a string, which can be either:
    ///
    /// - An HTML color code as accepted by [`Color::from_html`].
    /// - The name of a built-in color constant, such as `BLUE` or `lawn-green`. Matching is case-insensitive
    ///   and hyphens can be used interchangeably with underscores. See the [list of
    ///   color constants][color_constants] in the Godot API documentation, or the visual [cheat
    ///   sheet][cheat_sheet] for the full list.
    ///
    /// Returns `None` if the string is neither a valid HTML color code nor an existing color name.
    ///
    /// Most color constants have an alpha of 1; use [`Color::with_alpha`] to change it.
    ///
    /// [color_constants]: https://docs.godotengine.org/en/latest/classes/class_color.html#constants
    /// [cheat_sheet]: https://raw.githubusercontent.com/godotengine/godot-docs/master/img/color_constants.png
    ///
    /// _Godot equivalent: `Color.from_string()`_
    #[inline]
    pub fn from_string(string: impl AsArg<GString>) -> Option<Self> {
        arg_into_ref!(string);

        let color = InnerColor::from_string(
            string,
            Self::from_rgba(f32::NAN, f32::NAN, f32::NAN, f32::NAN),
        );

        // Assumption: the implementation of `from_string` in the engine will never return any NaN upon success.
        if color.r.is_nan() {
            None
        } else {
            Some(color)
        }
    }

    /// Constructs a `Color` from an [HSV profile](https://en.wikipedia.org/wiki/HSL_and_HSV).
    ///
    /// The hue (`h`), saturation (`s`), and value (`v`) are typically between 0.0 and 1.0. Alpha is set to 1; use [`Color::with_alpha`]
    /// to change it.
    ///
    /// See also: [`ColorHsv::to_rgb`] for fast conversion on Rust side.
    ///
    /// _Godot equivalent: `Color.from_hsv()`_
    #[inline]
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let h = h as f32;
        let s = s as f32;
        let v = v as f32;

        if s == 0.0 {
            return Color::from_rgb(v, v, v);
        }

        let h = (h * 6.0).rem_euclid(6.0);
        let i = h.floor() as i32;
        let f = h - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match i {
            0 => Color::from_rgb(v, t, p),
            1 => Color::from_rgb(q, v, p),
            2 => Color::from_rgb(p, v, t),
            3 => Color::from_rgb(p, q, v),
            4 => Color::from_rgb(t, p, v),
            _ => Color::from_rgb(v, p, q),
        }
    }

    /// Constructs a `Color` from an [OK HSL profile](https://bottosson.github.io/posts/colorpicker/).
    ///
    /// The hue (`h`), saturation (`s`), and lightness (`l`) are typically between 0.0 and 1.0. Alpha is set to 1; use
    /// [`Color::with_alpha`] to change it.
    ///
    /// _Godot equivalent: `Color.from_ok_hsl()`_
    #[inline]
    pub fn from_ok_hsl(h: f64, s: f64, l: f64) -> Self {
        // Source: https://bottosson.github.io/posts/colorpicker/
        //
        // This is a complex transformation involving multiple matrices and non-linearities.
        // For now, we delegate to the engine to ensure accuracy and avoid large constant matrices in code.
        InnerColor::from_ok_hsl(h, s, l, 1.0)
    }

    /// Constructs a `Color` from an RGBE9995 format integer. This is a special OpenGL texture
    /// format where the three color components have 9 bits of precision and all three share a
    /// single 5-bit exponent.
    ///
    /// _Godot equivalent: `Color.from_rgbe9995()`_
    #[inline]
    pub fn from_rgbe9995(rgbe: u32) -> Self {
        let r = (rgbe & 0x1ff) as f32;
        let g = ((rgbe >> 9) & 0x1ff) as f32;
        let b = ((rgbe >> 18) & 0x1ff) as f32;
        let e = ((rgbe >> 27) & 0x1f) as f32;
        let exp = 2.0f32.powf(e - 15.0 - 9.0);
        Color::from_rgb(r * exp, g * exp, b * exp)
    }

    /// Returns a copy of this color with the given alpha value. Useful for chaining with
    /// constructors like [`Color::from_string`] and [`Color::from_hsv`].
    #[must_use]
    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Returns the red channel value as a byte. If `self.r` is outside the range from 0 to 1, the
    /// returned byte is clamped.
    pub fn r8(self) -> u8 {
        to_u8(self.r)
    }

    /// Returns the green channel value as a byte. If `self.g` is outside the range from 0 to 1,
    /// the returned byte is clamped.
    pub fn g8(self) -> u8 {
        to_u8(self.g)
    }

    /// Returns the blue channel value as a byte. If `self.b` is outside the range from 0 to 1, the
    /// returned byte is clamped.
    pub fn b8(self) -> u8 {
        to_u8(self.b)
    }

    /// Returns the alpha channel value as a byte. If `self.a` is outside the range from 0 to 1,
    /// the returned byte is clamped.
    pub fn a8(self) -> u8 {
        to_u8(self.a)
    }

    /// Sets the red channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_r8(&mut self, r: u8) {
        self.r = from_u8(r);
    }

    /// Sets the green channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_g8(&mut self, g: u8) {
        self.g = from_u8(g);
    }

    /// Sets the blue channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_b8(&mut self, b: u8) {
        self.b = from_u8(b);
    }

    /// Sets the alpha channel value as a byte, mapped to the range from 0 to 1.
    pub fn set_a8(&mut self, a: u8) {
        self.a = from_u8(a);
    }

    /// Returns the light intensity of the color, as a value between 0.0 and 1.0 (inclusive).
    ///
    /// This is useful when determining whether a color is light or dark. Colors with a luminance smaller than 0.5 can be generally
    /// considered dark.
    ///
    /// Note: `luminance` relies on the color being in the linear color space to return an accurate relative luminance value. If the color
    /// is in the sRGB color space, use [`Color::srgb_to_linear`] to convert it to the linear color space first.
    ///
    /// _Godot equivalent: `Color.get_luminance()`_
    #[inline]
    pub fn luminance(self) -> f64 {
        f64::from(0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b)
    }

    /// Blends the given color on top of this color, taking its alpha into account.
    ///
    /// _Godot equivalent: `Color.blend()`_
    #[must_use]
    #[inline]
    pub fn blend(self, over: Color) -> Self {
        let mut res = Color::default();
        let sa = over.a;
        let da = self.a * (1.0 - sa);
        let a = sa + da;
        if a <= 0.0 {
            return Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        } else {
            res.r = (over.r * sa + self.r * da) / a;
            res.g = (over.g * sa + self.g * da) / a;
            res.b = (over.b * sa + self.b * da) / a;
            res.a = a;
        }
        res
    }

    /// Returns the linear interpolation between `self`'s components and `to`'s components. The
    /// interpolation factor `weight` should be between 0.0 and 1.0 (inclusive).
    ///
    /// _Godot equivalent: `Color.lerp()`_
    #[must_use]
    #[inline]
    pub fn lerp(self, to: Color, weight: f64) -> Self {
        let weight = weight as f32;
        Self {
            r: self.r + (to.r - self.r) * weight,
            g: self.g + (to.g - self.g) * weight,
            b: self.b + (to.b - self.b) * weight,
            a: self.a + (to.a - self.a) * weight,
        }
    }

    /// Returns a new color with all components clamped between the components of `min` and `max`.
    ///
    /// _Godot equivalent: `Color.clamp()`_
    #[must_use]
    #[inline]
    pub fn clamp(self, min: Color, max: Color) -> Self {
        Color {
            r: self.r.clamp(min.r, max.r),
            g: self.g.clamp(min.g, max.g),
            b: self.b.clamp(min.b, max.b),
            a: self.a.clamp(min.a, max.a),
        }
    }

    /// Creates a new color resulting by making this color darker by the specified amount (ratio
    /// from 0.0 to 1.0). See also [`lightened`][Self::lightened].
    ///
    /// _Godot equivalent: `Color.darkened()`_
    #[must_use]
    #[inline]
    pub fn darkened(self, amount: f64) -> Self {
        let amount = amount as f32;
        Color {
            r: self.r * (1.0 - amount),
            g: self.g * (1.0 - amount),
            b: self.b * (1.0 - amount),
            a: self.a,
        }
    }

    /// Creates a new color resulting by making this color lighter by the specified amount, which
    /// should be a ratio from 0.0 to 1.0. See also [`darkened`][Self::darkened].
    ///
    /// _Godot equivalent: `Color.lightened()`_
    #[must_use]
    #[inline]
    pub fn lightened(self, amount: f64) -> Self {
        let amount = amount as f32;
        Color {
            r: self.r + (1.0 - self.r) * amount,
            g: self.g + (1.0 - self.g) * amount,
            b: self.b + (1.0 - self.b) * amount,
            a: self.a,
        }
    }

    /// Returns the color with its `r`, `g`, and `b` components inverted:
    /// `Color::from_rgba(1 - r, 1 - g, 1 - b, a)`.
    ///
    /// _Godot equivalent: `Color.inverted()`_
    #[must_use]
    #[inline]
    pub fn inverted(self) -> Self {
        Color {
            r: 1.0 - self.r,
            g: 1.0 - self.g,
            b: 1.0 - self.b,
            a: self.a,
        }
    }

    /// Returns the color converted to the [sRGB](https://en.wikipedia.org/wiki/SRGB) color space.
    ///
    /// This method assumes the original color is in the linear color space. See also
    /// [`Color::srgb_to_linear`] which performs the opposite operation.
    /// This method assumes the original color is in the linear color space. See also
    /// [`Color::srgb_to_linear`] which performs the opposite operation.
    ///
    /// _Godot equivalent: `Color.linear_to_srgb()`_
    #[must_use]
    #[inline]
    pub fn linear_to_srgb(self) -> Self {
        fn f(c: f32) -> f32 {
            if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        }
        Color {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: self.a,
        }
    }

    /// Returns the color converted to the linear color space.
    ///
    /// This method assumes the original color is in the sRGB color space. See also [`Color::linear_to_srgb`]
    /// which performs the opposite operation.
    ///
    /// _Godot equivalent: `Color.srgb_to_linear()`_
    #[must_use]
    #[inline]
    pub fn srgb_to_linear(self) -> Self {
        fn f(c: f32) -> f32 {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        Color {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: self.a,
        }
    }

    /// Returns the HTML color code representation of this color, as 8 lowercase hex digits in the
    /// order `RRGGBBAA`, without the `#` prefix.
    /// order `RRGGBBAA`, without the `#` prefix.
    ///
    /// _Godot equivalent: `Color.to_html(true)`_
    #[inline]
    pub fn to_html(self) -> GString {
        format!(
            "{:02x}{:02x}{:02x}{:02x}",
            self.r8(),
            self.g8(),
            self.b8(),
            self.a8()
        )
        .into()
    }

    /// Returns the HTML color code representation of this color, as 6 lowercase hex digits in the
    /// order `RRGGBB`, without the `#` prefix. The alpha channel is ignored.
    ///
    /// _Godot equivalent: `Color.to_html(false)`_
    #[inline]
    pub fn to_html_without_alpha(self) -> GString {
        format!("{:02x}{:02x}{:02x}", self.r8(), self.g8(), self.b8()).into()
    }

    /// Returns the color converted to a 32-bit integer (each component is 8 bits) with the given
    /// `order` of channels (from most to least significant byte).
    pub fn to_u32(self, order: ColorChannelOrder) -> u32 {
        u32::from_be_bytes(order.pack([to_u8(self.r), to_u8(self.g), to_u8(self.b), to_u8(self.a)]))
    }

    /// Returns the color converted to a 64-bit integer (each component is 16 bits) with the given
    /// `order` of channels (from most to least significant word).
    pub fn to_u64(self, order: ColorChannelOrder) -> u64 {
        from_be_words(order.pack([
            to_u16(self.r),
            to_u16(self.g),
            to_u16(self.b),
            to_u16(self.a),
        ]))
    }

    /// ⚠️ Convert `Color` into [`ColorHsv`].
    ///
    /// # Panics
    ///
    /// Method will panic if the RGBA values are outside the valid range `0.0..=1.0`. You can use [`Color::normalized`] to ensure that
    /// they are in range, or use [`Color::try_to_hsv`].
    #[inline]
    #[track_caller]
    pub fn to_hsv(self) -> ColorHsv {
        self.assert_normalized();
        let (h, s, v, a) = rgba_to_hsva(self.r, self.g, self.b, self.a);
        ColorHsv { h, s, v, a }
    }

    /// Fallible `Color` conversion into [`ColorHsv`]. See also [`Color::to_hsv`].
    pub fn try_to_hsv(self) -> Result<ColorHsv, String> {
        if !self.is_normalized() {
            return Err(format!("RGBA values need to be in range `0.0..=1.0` before conversion, but were {self:?}. See: `Color::normalized()` method."));
        }
        let (h, s, v, a) = rgba_to_hsva(self.r, self.g, self.b, self.a);

        Ok(ColorHsv { h, s, v, a })
    }

    /// Clamps all components to a usually valid range `0.0..=1.0`.
    ///
    /// Useful for transformations between different color representations.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    // For internal checks before transformations between different color representation.
    pub(crate) fn is_normalized(&self) -> bool {
        self.r >= 0.0
            && self.r <= 1.0
            && self.g >= 0.0
            && self.g <= 1.0
            && self.b >= 0.0
            && self.b <= 1.0
            && self.a >= 0.0
            && self.a <= 1.0
    }

    #[inline]
    #[track_caller]
    pub fn assert_normalized(&self) {
        assert!(
            self.is_normalized(),
            "{}: RGBA values need to be in range `0.0..=1.0` before conversion, but were {:?}. See: `Color::normalized()` method.",
            std::any::type_name::<Self>(),
            self
        );
    }

    /// Returns `true` if this color and `other` are approximately equal.
    #[inline]
    pub fn is_equal_approx(self, other: Self) -> bool {
        self.approx_eq(&other)
    }
}

// SAFETY:
// This type is represented as `Self` in Godot, so `*mut Self` is sound.
unsafe impl GodotFfi for Color {
    const VARIANT_TYPE: ExtVariantType = ExtVariantType::Concrete(sys::VariantType::COLOR);

    ffi_methods! { type sys::GDExtensionTypePtr = *mut Self; .. }
}

crate::meta::impl_godot_as_self!(Color: ByValue);

impl ApproxEq for Color {
    fn approx_eq(&self, other: &Self) -> bool {
        self.r.approx_eq(&other.r)
            && self.g.approx_eq(&other.g)
            && self.b.approx_eq(&other.b)
            && self.a.approx_eq(&other.a)
    }
}

impl_geometric_interop!(Color, (f32, f32, f32, f32),
    [f32; 4], from_rgba, [r, g, b, a], self => [self.r, self.g, self.b, self.a]);

/// Defines how individual color channels are laid out in memory.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ColorChannelOrder {
    /// RGBA channel order. Godot's default.
    RGBA,

    /// ABGR channel order. Reverse of the default RGBA order.
    ABGR,

    /// ARGB channel order. More compatible with DirectX.
    ARGB,
}

impl ColorChannelOrder {
    const fn pack<T: Copy>(self, rgba: [T; 4]) -> [T; 4] {
        let [r, g, b, a] = rgba;
        match self {
            ColorChannelOrder::RGBA => [r, g, b, a],
            ColorChannelOrder::ABGR => [a, b, g, r],
            ColorChannelOrder::ARGB => [a, r, g, b],
        }
    }

    const fn unpack<T: Copy>(self, xyzw: [T; 4]) -> [T; 4] {
        let [x, y, z, w] = xyzw;
        match self {
            ColorChannelOrder::RGBA => [x, y, z, w],
            ColorChannelOrder::ABGR => [w, z, y, x],
            ColorChannelOrder::ARGB => [y, z, w, x],
        }
    }
}

/// Constructs a default `Color` which is opaque black.
impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(mut self, rhs: Color) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, mut rhs: Color) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, f: f32) {
        self.r *= f;
        self.g *= f;
        self.b *= f;
        self.a *= f;
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, f: f32) {
        self.r /= f;
        self.g /= f;
        self.b /= f;
        self.a /= f;
    }
}

impl ops::Div<Color> for Color {
    type Output = Color;
    fn div(mut self, rhs: Color) -> Self::Output {
        self /= rhs;
        self
    }
}

impl ops::DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl ops::Rem<Color> for Color {
    type Output = Color;
    fn rem(mut self, rhs: Color) -> Self::Output {
        self %= rhs;
        self
    }
}

impl ops::RemAssign<Color> for Color {
    fn rem_assign(&mut self, rhs: Color) {
        self.r %= rhs.r;
        self.g %= rhs.g;
        self.b %= rhs.b;
        self.a %= rhs.a;
    }
}

impl ops::Rem<f32> for Color {
    type Output = Color;
    fn rem(mut self, rhs: f32) -> Self::Output {
        self %= rhs;
        self
    }
}

impl ops::RemAssign<f32> for Color {
    fn rem_assign(&mut self, rhs: f32) {
        self.r %= rhs;
        self.g %= rhs;
        self.b %= rhs;
        self.a %= rhs;
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(mut self, rhs: Color) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(mut self, rhs: Color) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl ops::Neg for Color {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_rgba(-self.r, -self.g, -self.b, -self.a)
    }
}

/// Converts a single channel byte to a float in the range 0 to 1.
const fn from_u8(byte: u8) -> f32 {
    byte as f32 / 255.0
}

/// Converts a single channel `u16` word to a float in the range 0 to 1.
const fn from_u16(byte: u16) -> f32 {
    byte as f32 / 65535.0
}

/// Converts a float in the range 0 to 1 to a byte. Matches rounding behavior of the engine.
fn to_u8(v: f32) -> u8 {
    // core/math/color.h:
    // _FORCE_INLINE_ int32_t get_r8() const { return int32_t(CLAMP(Math::round(r * 255.0f), 0.0f, 255.0f)); }
    const MAX: f32 = 255.0;
    (v * MAX).round().clamp(0.0, MAX) as u8
}

/// Converts a float in the range 0 to 1 to a `u16` word. Matches rounding behavior of the engine.
fn to_u16(v: f32) -> u16 {
    // core/math/color.cpp:
    // uint64_t c = (uint16_t)Math::round(a * 65535.0f);
    // It does not clamp, but we do.
    const MAX: f32 = 65535.0;
    (v * MAX).round().clamp(0.0, MAX) as u16
}

/// Packs four `u16` words into a `u64` in big-endian order.
fn from_be_words(words: [u16; 4]) -> u64 {
    (words[0] as u64) << 48 | (words[1] as u64) << 32 | (words[2] as u64) << 16 | (words[3] as u64)
}

/// Unpacks a `u64` into four `u16` words in big-endian order.
const fn to_be_words(mut u: u64) -> [u16; 4] {
    let w = (u & 0xffff) as u16;
    u >>= 16;
    let z = (u & 0xffff) as u16;
    u >>= 16;
    let y = (u & 0xffff) as u16;
    u >>= 16;
    let x = (u & 0xffff) as u16;
    [x, y, z, w]
}

impl std::fmt::Display for Color {
    /// Formats `Color` to match Godot's string representation.
    ///
    /// # Example
    /// ```
    /// use godot::prelude::*;
    /// let color = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    /// assert_eq!(format!("{}", color), "(1, 1, 1, 1)");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arithmetic() {
        use crate::assert_eq_approx;

        let c1 = Color::from_rgba(0.2, 0.4, 0.6, 0.8);
        let c2 = Color::from_rgba(0.1, 0.2, 0.3, 0.4);

        assert_eq_approx!(c1 + c2, Color::from_rgba(0.3, 0.6, 0.9, 1.2));
        assert_eq_approx!(c1 - c2, Color::from_rgba(0.1, 0.2, 0.3, 0.4));
        assert_eq_approx!(c1 * c2, Color::from_rgba(0.02, 0.08, 0.18, 0.32));
        assert_eq_approx!(c1 / c2, Color::from_rgba(2.0, 2.0, 2.0, 2.0));

        assert_eq_approx!(c1 * 2.0, Color::from_rgba(0.4, 0.8, 1.2, 1.6));
        assert_eq_approx!(2.0 * c1, Color::from_rgba(0.4, 0.8, 1.2, 1.6));
        assert_eq_approx!(c1 / 2.0, Color::from_rgba(0.1, 0.2, 0.3, 0.4));
        assert_eq_approx!(c1 % c2, Color::from_rgba(0.0, 0.0, 0.0, 0.0));
        assert_eq_approx!(c1 % 0.3, Color::from_rgba(0.2, 0.1, 0.0, 0.2));

        let mut c = c1;
        c += c2;
        assert_eq_approx!(c, c1 + c2);

        let mut c = c1;
        c -= c2;
        assert_eq_approx!(c, c1 - c2);

        let mut c = c1;
        c *= 2.0;
        assert_eq_approx!(c, c1 * 2.0);

        let mut c = c1;
        c /= 2.0;
        assert_eq_approx!(c, c1 / 2.0);

        let mut c = c1;
        c %= c2;
        assert_eq_approx!(c, c1 % c2);

        let mut c = c1;
        c %= 0.3;
        assert_eq_approx!(c, c1 % 0.3);

        assert_eq_approx!(-c1, Color::from_rgba(-0.2, -0.4, -0.6, -0.8));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let color = super::Color::WHITE;
        let expected_json = "{\"r\":1.0,\"g\":1.0,\"b\":1.0,\"a\":1.0}";

        crate::builtin::test_utils::roundtrip(&color, expected_json);
    }
}
