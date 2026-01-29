/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use godot::builtin::math::assert_eq_approx;
use crate::framework::itest;

#[itest]
fn vector_approx_eq() {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(1.0000001, 2.0);
    assert_eq_approx!(v1, v2);

    let v3 = Vector3::new(1.0, 2.0, 3.0);
    let v4 = Vector3::new(1.0, 2.0000001, 3.0);
    assert_eq_approx!(v3, v4);

    let v5 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let v6 = Vector4::new(1.0, 2.0, 3.0000001, 4.0);
    assert_eq_approx!(v5, v6);
}

#[itest]
fn basis_approx_eq() {
    let b1 = Basis::IDENTITY;
    let b2 = Basis::from_scale(Vector3::new(1.0000001, 1.0, 1.0));
    assert_eq_approx!(b1, b2);
}

#[itest]
fn quaternion_approx_eq() {
    let q1 = Quaternion::IDENTITY;
    let q2 = Quaternion::new(0.0000001, 0.0, 0.0, 1.0);
    assert_eq_approx!(q1, q2);
}

#[itest]
fn color_approx_eq() {
    let c1 = Color::from_rgb(1.0, 0.5, 0.2);
    let c2 = Color::from_rgb(1.0000001, 0.5, 0.2);
    assert_eq_approx!(c1, c2);
}

#[itest]
fn projection_approx_eq() {
    let p1 = Projection::IDENTITY;
    let mut p2 = Projection::IDENTITY;
    p2.cols[0].x = 1.0000001;
    assert_eq_approx!(p1, p2);
}

#[itest]
fn transform_approx_eq() {
    let t1 = Transform2D::IDENTITY;
    let t2 = Transform2D::from_angle_origin(0.0000001, Vector2::ZERO);
    assert_eq_approx!(t1, t2);

    let t3 = Transform3D::IDENTITY;
    let t4 = Transform3D::new(Basis::from_scale(Vector3::new(1.0000001, 1.0, 1.0)), Vector3::ZERO);
    assert_eq_approx!(t3, t4);
}
