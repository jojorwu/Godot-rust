/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::sys;
use std::fmt;

/// An error that can occur during a method call, containing details about the failure.
///
/// See [`MethodInfo::call`][crate::meta::MethodInfo::call].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CallError {
    /// The specific type of error that occurred.
    pub error: CallErrorType,
    /// The index of the argument that caused the error, if applicable.
    pub argument: i32,
    /// The expected value or type, depending on the error context.
    pub expected: i32,
}

/// The specific type of error in a [`CallError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CallErrorType {
    /// The method call was successful.
    Ok,
    /// The method being called is invalid.
    InvalidMethod,
    /// An argument is of the wrong type.
    InvalidArgument,
    /// Too many arguments were passed to the method.
    TooManyArguments,
    /// Too few arguments were passed to the method.
    TooFewArguments,
    /// The object instance is null.
    InstanceIsNull,
    /// The method is not constant and is being called on a constant object.
    MethodNotConst,
}

impl CallError {
    pub(crate) fn from_sys(sys_error: sys::GDExtensionCallError) -> Self {
        let error_type = match sys_error.error {
            sys::GDEXTENSION_CALL_OK => CallErrorType::Ok,
            sys::GDEXTENSION_CALL_ERROR_INVALID_METHOD => CallErrorType::InvalidMethod,
            sys::GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT => CallErrorType::InvalidArgument,
            sys::GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS => CallErrorType::TooManyArguments,
            sys::GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS => CallErrorType::TooFewArguments,
            sys::GDEXTENSION_CALL_ERROR_INSTANCE_IS_NULL => CallErrorType::InstanceIsNull,
            sys::GDEXTENSION_CALL_ERROR_METHOD_NOT_CONST => CallErrorType::MethodNotConst,
            _ => {
                // This should not be reached if the enum is kept in sync with Godot.
                // However, if Godot adds new error types, this will prevent a crash.
                // We might want to log a warning here in the future.
                CallErrorType::InvalidMethod
            }
        };

        Self {
            error: error_type,
            argument: sys_error.argument,
            expected: sys_error.expected,
        }
    }
}

impl fmt::Debug for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("CallError");
        f.field("error", &self.error);
        if self.argument != 0 || self.expected != 0 {
            f.field("argument", &self.argument);
            f.field("expected", &self.expected);
        }
        f.finish()
    }
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            CallErrorType::Ok => write!(f, "Method call was successful"),
            CallErrorType::InvalidMethod => write!(f, "Invalid method"),
            CallErrorType::InvalidArgument => write!(f, "Invalid argument {} (expected {})", self.argument, self.expected),
            CallErrorType::TooManyArguments => write!(f, "Too many arguments (expected {})", self.expected),
            CallErrorType::TooFewArguments => write!(f, "Too few arguments (expected {})", self.expected),
            CallErrorType::InstanceIsNull => write!(f, "Instance is null"),
            CallErrorType::MethodNotConst => write!(f, "Method is not constant"),
        }
    }
}

impl std::error::Error for CallError {}
