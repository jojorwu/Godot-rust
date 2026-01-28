/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error;
use std::fmt;

use godot_ffi::VariantType;

use crate::builtin::Variant;
use crate::meta::{ClassId, ElementType, ToGodot};

type Cause = Box<dyn Error + Send + Sync>;

/// A thread-safe representation of a value that failed to convert.
#[derive(Debug, Clone)]
pub(crate) enum ThreadSafeValue {
    Int(i64),
    #[allow(dead_code)]
    Real(f64),
    #[allow(dead_code)]
    String(String),
}

impl fmt::Display for ThreadSafeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::Real(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v:?}"),
        }
    }
}

/// Represents errors that can occur when converting values from Godot.
///
/// To create user-defined errors, you can use [`ConvertError::default()`] or [`ConvertError::new("message")`][Self::new].
#[derive(Debug)]
pub struct ConvertError {
    kind: ErrorKind,
    value: Option<Variant>,
    thread_safe_value: Option<ThreadSafeValue>,
}

impl ConvertError {
    /// Construct with a user-defined message.
    ///
    /// If you don't need a custom message, consider using [`ConvertError::default()`] instead.
    pub fn new(user_message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Custom(Some(user_message.into().into())),
            ..Default::default()
        }
    }

    /// Create a new custom error for a conversion, without associated value.
    #[allow(dead_code)] // Needed a few times already, stays to prevent churn on refactorings.
    pub(crate) fn with_kind(kind: ErrorKind) -> Self {
        Self {
            kind,
            value: None,
            thread_safe_value: None,
        }
    }

    /// Create a new custom error for a conversion with the value that failed to convert.
    pub(crate) fn with_kind_value<V>(kind: ErrorKind, value: V) -> Self
    where
        V: ToGodot,
    {
        Self {
            kind,
            value: Some(value.to_variant()),
            thread_safe_value: None,
        }
    }

    pub(crate) fn with_kind_thread_safe_value<V>(
        kind: ErrorKind,
        value: V,
        ts_value: ThreadSafeValue,
    ) -> Self
    where
        V: ToGodot,
    {
        Self {
            kind,
            value: Some(value.to_variant()),
            thread_safe_value: Some(ts_value),
        }
    }

    /// Create a new custom error wrapping an [`Error`].
    pub fn with_error<E>(error: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Custom(Some(error.into())),
            ..Default::default()
        }
    }

    /// Create a new custom error wrapping an [`Error`] and the value that failed to convert.
    pub fn with_error_value<E, V>(error: E, value: V) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
        V: ToGodot,
    {
        Self {
            kind: ErrorKind::Custom(Some(error.into())),
            value: Some(value.to_variant()),
            thread_safe_value: None,
        }
    }

    /// Returns the rust-error that caused this error, if one exists.
    pub fn cause(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        match &self.kind {
            ErrorKind::Custom(Some(cause)) => Some(&**cause),
            _ => None,
        }
    }

    /// Returns a reference of the value that failed to convert, if one exists.
    pub fn value(&self) -> Option<&Variant> {
        self.value.as_ref()
    }

    /// Converts error into generic error type. It is useful to send error across thread.
    /// Do note that some data might get lost during conversion.
    pub fn into_erased(self) -> impl Error + Send + Sync {
        ErasedConvertError::from(self)
    }

    #[cfg(before_api = "4.4")]
    pub(crate) fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(value) = &self.value {
            write!(f, ": {value:?}")?;
        } else if let Some(ts_value) = &self.thread_safe_value {
            write!(f, ": {ts_value}")?;
        }

        Ok(())
    }
}

impl Error for ConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause().map(|v| v as &(dyn Error + 'static))
    }
}

impl Default for ConvertError {
    /// Create a custom error, without any description.
    ///
    /// If you need a custom message, consider using [`ConvertError::new("message")`][Self::new] instead.
    fn default() -> Self {
        Self {
            kind: ErrorKind::Custom(None),
            value: None,
            thread_safe_value: None,
        }
    }
}

/// Erased type of [`ConvertError`].
#[derive(Debug)]
pub(crate) struct ErasedConvertError {
    kind: ErrorKind,
    thread_safe_value: Option<ThreadSafeValue>,
}

impl From<ConvertError> for ErasedConvertError {
    fn from(v: ConvertError) -> Self {
        let ConvertError {
            kind,
            thread_safe_value,
            ..
        } = v;
        Self {
            kind,
            thread_safe_value,
        }
    }
}

impl fmt::Display for ErasedConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(ts_value) = &self.thread_safe_value {
            write!(f, ": {ts_value}")?;
        }

        Ok(())
    }
}

impl Error for ErasedConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ErrorKind::Custom(Some(cause)) => Some(&**cause),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    FromGodot(FromGodotError),
    FromFfi(FromFfiError),
    FromVariant(FromVariantError),
    // FromAnyArray(ArrayMismatch), -- needed if AnyArray downcasts return ConvertError one day.
    Custom(Option<Cause>),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromGodot(from_godot) => write!(f, "{from_godot}"),
            Self::FromVariant(from_variant) => write!(f, "{from_variant}"),
            Self::FromFfi(from_ffi) => write!(f, "{from_ffi}"),
            Self::Custom(cause) => match cause {
                Some(c) => write!(f, "{c}"),
                None => write!(f, "custom error"),
            },
        }
    }
}

/// Conversion failed during a [`FromGodot`](crate::meta::FromGodot) call.
#[derive(Eq, PartialEq, Debug)]
pub(crate) enum FromGodotError {
    /// Destination `Array<T>` has different type than source's runtime type.
    BadArrayType(ArrayMismatch),

    /// Special case of `BadArrayType` where a custom int type such as `i8` cannot hold a dynamic `i64` value.
    #[cfg(safeguards_strict)]
    BadArrayTypeInt {
        expected_int_type: &'static str,
        value: i64,
    },

    /// InvalidEnum is also used by bitfields.
    InvalidEnum,

    /// Cannot map object to `dyn Trait` because none of the known concrete classes implements it.
    UnimplementedDynTrait {
        trait_name: String,
        class_name: String,
    },

    /// Cannot map object to `dyn Trait` because none of the known concrete classes implements it.
    UnregisteredDynTrait { trait_name: String },

    /// `InstanceId` cannot be 0.
    ZeroInstanceId,
}

impl FromGodotError {
    pub fn into_error<V>(self, value: V) -> ConvertError
    where
        V: ToGodot,
    {
        ConvertError::with_kind_value(ErrorKind::FromGodot(self), value)
    }
}

impl fmt::Display for FromGodotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadArrayType(mismatch) => write!(f, "{mismatch}"),

            #[cfg(safeguards_strict)]
            Self::BadArrayTypeInt {
                expected_int_type,
                value,
            } => {
                write!(
                    f,
                    "integer value {value} does not fit into Array<{expected_int_type}>"
                )
            }

            Self::InvalidEnum => write!(f, "invalid engine enum value"),

            Self::ZeroInstanceId => write!(f, "`InstanceId` cannot be 0"),

            Self::UnimplementedDynTrait {
                trait_name,
                class_name,
            } => {
                write!(
                    f,
                    "none of the classes derived from `{class_name}` have been linked to trait `{trait_name}` with #[godot_dyn]"
                )
            }

            FromGodotError::UnregisteredDynTrait { trait_name } => {
                write!(
                    f,
                    "trait `{trait_name}` has not been registered with #[godot_dyn]"
                )
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct ArrayMismatch {
    pub expected: ElementType,
    pub actual: ElementType,
}

impl fmt::Display for ArrayMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ArrayMismatch { expected, actual } = self;

        if expected.variant_type() != actual.variant_type() {
            return write!(f, "expected array of type {expected:?}, got {actual:?}");
        }

        let exp_class = format!("{expected:?}");
        let act_class = format!("{actual:?}");

        write!(f, "expected array of type {exp_class}, got {act_class}")
    }
}

/// Conversion failed during a [`GodotType::try_from_ffi()`](crate::meta::GodotType::try_from_ffi()) call.
#[derive(Eq, PartialEq, Debug)]
#[non_exhaustive]
pub(crate) enum FromFfiError {
    NullRawGd,
    WrongObjectType,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
}

impl FromFfiError {
    pub fn into_error<V>(self, value: V) -> ConvertError
    where
        V: ToGodot,
    {
        ConvertError::with_kind_value(ErrorKind::FromFfi(self), value)
    }

    pub fn into_error_ts<V>(self, value: V, ts_value: ThreadSafeValue) -> ConvertError
    where
        V: ToGodot,
    {
        ConvertError::with_kind_thread_safe_value(ErrorKind::FromFfi(self), value, ts_value)
    }
}

impl fmt::Display for FromFfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target = match self {
            Self::NullRawGd => return write!(f, "`Gd` cannot be null"),
            Self::WrongObjectType => {
                return write!(f, "given object cannot be cast to target type")
            }
            Self::I8 => "i8",
            Self::U8 => "u8",
            Self::I16 => "i16",
            Self::U16 => "u16",
            Self::I32 => "i32",
            Self::U32 => "u32",
        };

        write!(f, "`{target}` cannot store the given value")
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum FromVariantError {
    /// Variant type does not match expected type.
    BadType {
        expected: VariantType,
        actual: VariantType,
    },

    WrongClass {
        expected: ClassId,
        actual: Option<String>,
    },

    /// Variant holds an object which is no longer alive.
    DeadObject,
    //
    // BadValue: Value cannot be represented in target type's domain.
    // Used in the past for types like u64, with fallible FromVariant.
}

impl FromVariantError {
    pub fn into_error<V>(self, value: V) -> ConvertError
    where
        V: ToGodot,
    {
        ConvertError::with_kind_value(ErrorKind::FromVariant(self), value)
    }
}

impl fmt::Display for FromVariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadType { expected, actual } => {
                // Note: wording is the same as in CallError::failed_param_conversion_engine()
                write!(f, "cannot convert from {actual:?} to {expected:?}")
            }
            Self::WrongClass { expected, actual } => {
                write!(f, "cannot convert to class {expected}")?;
                if let Some(actual) = actual {
                    write!(f, " (found {actual})")?;
                }
                Ok(())
            }
            Self::DeadObject => write!(f, "variant holds object which is no longer alive"),
        }
    }
}

fn __ensure_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<ErasedConvertError>();
}
