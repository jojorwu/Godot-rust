/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

macro_rules! impl_owned_rid {
    ($name:ident, $server:ident, instance, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Eq, PartialEq, Hash)]
        pub struct $name {
            rid: crate::builtin::Rid,
            server: crate::obj::Gd<crate::classes::$server>,
        }

        impl $name {
            /// Returns the underlying RID of the resource.
            pub fn rid(&self) -> crate::builtin::Rid {
                self.rid
            }

            /// Returns the server that manages this resource.
            pub fn server(&self) -> crate::obj::Gd<crate::classes::$server> {
                self.server.clone()
            }
        }

        impl std::ops::Deref for $name {
            type Target = crate::builtin::Rid;

            fn deref(&self) -> &Self::Target {
                &self.rid
            }
        }

        impl AsRef<crate::builtin::Rid> for $name {
            fn as_ref(&self) -> &crate::builtin::Rid {
                &self.rid
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.rid.is_valid() {
                    self.server.clone().free_rid(self.rid);
                }
            }
        }
    };
    ($name:ident, $server:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Eq, PartialEq, Hash)]
        pub struct $name {
            rid: crate::builtin::Rid,
        }

        impl $name {
            /// Returns the underlying RID of the resource.
            pub fn rid(&self) -> crate::builtin::Rid {
                self.rid
            }
        }

        impl std::ops::Deref for $name {
            type Target = crate::builtin::Rid;

            fn deref(&self) -> &Self::Target {
                &self.rid
            }
        }

        impl AsRef<crate::builtin::Rid> for $name {
            fn as_ref(&self) -> &crate::builtin::Rid {
                &self.rid
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.rid.is_valid() {
                    use crate::obj::Singleton as _;
                    crate::classes::$server::singleton().free_rid(self.rid);
                }
            }
        }
    };
    ($name:ident, $doc:literal) => {
        crate::obj::impl_owned_rid!($name, RenderingServer, $doc);
    };
}

pub(crate) use impl_owned_rid;
