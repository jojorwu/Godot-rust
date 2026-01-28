/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `PhysicsServer2D` and `PhysicsServer3D` resources.

pub(crate) use crate::obj::impl_owned_rid;

pub mod owned_area_2d;
pub mod owned_body_2d;
pub mod owned_shape_2d;
pub mod owned_space_2d;

pub mod owned_area_3d;
pub mod owned_body_3d;
pub mod owned_shape_3d;
pub mod owned_soft_body_3d;
pub mod owned_space_3d;

pub use owned_area_2d::OwnedArea2D;
pub use owned_body_2d::OwnedBody2D;
pub use owned_shape_2d::OwnedShape2D;
pub use owned_space_2d::OwnedSpace2D;

pub use owned_area_3d::OwnedArea3D;
pub use owned_body_3d::OwnedBody3D;
pub use owned_shape_3d::OwnedShape3D;
pub use owned_soft_body_3d::OwnedSoftBody3D;
pub use owned_space_3d::OwnedSpace3D;

impl crate::classes::PhysicsServer2D {
    /// Creates a new space and returns a wrapper that will free it on drop.
    pub fn space_create_owned(&mut self) -> OwnedSpace2D {
        OwnedSpace2D::new()
    }

    /// Creates a new area and returns a wrapper that will free it on drop.
    pub fn area_create_owned(&mut self) -> OwnedArea2D {
        OwnedArea2D::new()
    }

    /// Creates a new body and returns a wrapper that will free it on drop.
    pub fn body_create_owned(&mut self) -> OwnedBody2D {
        OwnedBody2D::new()
    }

    /// Creates a new shape and returns a wrapper that will free it on drop.
    pub fn shape_create_owned(
        &mut self,
        shape_type: crate::classes::physics_server_2d::ShapeType,
    ) -> OwnedShape2D {
        OwnedShape2D::new(shape_type)
    }
}

impl crate::classes::PhysicsServer3D {
    /// Creates a new space and returns a wrapper that will free it on drop.
    pub fn space_create_owned(&mut self) -> OwnedSpace3D {
        OwnedSpace3D::new()
    }

    /// Creates a new area and returns a wrapper that will free it on drop.
    pub fn area_create_owned(&mut self) -> OwnedArea3D {
        OwnedArea3D::new()
    }

    /// Creates a new body and returns a wrapper that will free it on drop.
    pub fn body_create_owned(&mut self) -> OwnedBody3D {
        OwnedBody3D::new()
    }

    /// Creates a new soft body and returns a wrapper that will free it on drop.
    pub fn soft_body_create_owned(&mut self) -> OwnedSoftBody3D {
        OwnedSoftBody3D::new()
    }

    /// Creates a new shape and returns a wrapper that will free it on drop.
    pub fn shape_create_owned(
        &mut self,
        shape_type: crate::classes::physics_server_3d::ShapeType,
    ) -> OwnedShape3D {
        OwnedShape3D::new(shape_type)
    }
}
