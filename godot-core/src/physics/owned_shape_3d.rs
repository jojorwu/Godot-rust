/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes::physics_server_3d::ShapeType;
use crate::classes::PhysicsServer3D;
use crate::obj::Singleton;

crate::physics::impl_owned_rid!(
    OwnedShape3D,
    PhysicsServer3D,
    "A RAII wrapper for a 3D physics shape RID that is owned by this type.\nThe shape is freed when this object is dropped."
);

impl OwnedShape3D {
    /// Creates a new shape of the given type and returns a wrapper that will free it on drop.
    ///
    /// See `PhysicsServer3D.shape_create()`.
    pub fn new(shape_type: ShapeType) -> Self {
        let mut server = PhysicsServer3D::singleton();
        let rid = match shape_type {
            ShapeType::WORLD_BOUNDARY => server.world_boundary_shape_create(),
            ShapeType::SEPARATION_RAY => server.separation_ray_shape_create(),
            ShapeType::SPHERE => server.sphere_shape_create(),
            ShapeType::BOX => server.box_shape_create(),
            ShapeType::CAPSULE => server.capsule_shape_create(),
            ShapeType::CYLINDER => server.cylinder_shape_create(),
            ShapeType::CONVEX_POLYGON => server.convex_polygon_shape_create(),
            ShapeType::CONCAVE_POLYGON => server.concave_polygon_shape_create(),
            ShapeType::HEIGHTMAP => server.heightmap_shape_create(),
            ShapeType::CUSTOM => server.custom_shape_create(),
            _ => panic!("Unsupported shape type"),
        };
        Self { rid }
    }
}
