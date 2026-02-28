/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::{Vector2, Vector3};

/// A library of static functions for 2D geometric operations.
///
/// Corresponds to Godot's `Geometry2D` singleton, but implemented in native Rust for performance.
pub struct Geometry2D;

impl Geometry2D {
    /// Returns `true` if `point` is inside the circle or on its boundary.
    ///
    /// _Godot equivalent: `is_point_in_circle`_
    pub fn is_point_in_circle(point: Vector2, circle_position: Vector2, circle_radius: f32) -> bool {
        point.distance_to(circle_position) <= circle_radius
    }

    /// Returns `true` if `point` is inside the triangle or on its boundary.
    ///
    /// _Godot equivalent: `is_point_in_triangle`_
    pub fn is_point_in_triangle(point: Vector2, a: Vector2, b: Vector2, c: Vector2) -> bool {
        let b0 = (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x) > 0.0;
        let b1 = (c.x - b.x) * (point.y - b.y) - (c.y - b.y) * (point.x - b.x) > 0.0;
        let b2 = (a.x - c.x) * (point.y - c.y) - (a.y - c.y) * (point.x - c.x) > 0.0;

        (b0 == b1) && (b1 == b2)
    }

    /// Checks if the two segments (`from_a`, `to_a`) and (`from_b`, `to_b`) intersect.
    /// If they do, returns the intersection point.
    ///
    /// _Godot equivalent: `segment_intersects_segment`_
    pub fn segment_intersects_segment(from_a: Vector2, to_a: Vector2, from_b: Vector2, to_b: Vector2) -> Option<Vector2> {
        let det = (to_a.x - from_a.x) * (to_b.y - from_b.y) - (to_a.y - from_a.y) * (to_b.x - from_b.x);
        if det.abs() < 1e-6 {
            return None;
        }

        let t = ((from_b.x - from_a.x) * (to_b.y - from_b.y) - (from_b.y - from_a.y) * (to_b.x - from_b.x)) / det;
        let u = ((from_b.x - from_a.x) * (to_a.y - from_a.y) - (from_b.y - from_a.y) * (to_a.x - from_a.x)) / det;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            Some(Vector2::new(from_a.x + t * (to_a.x - from_a.x), from_a.y + t * (to_a.y - from_a.y)))
        } else {
            None
        }
    }

    /// Returns the point on the segment (`from`, `to`) that is closest to `point`.
    ///
    /// _Godot equivalent: `get_closest_point_to_segment`_
    pub fn get_closest_point_to_segment(point: Vector2, from: Vector2, to: Vector2) -> Vector2 {
        let p = point - from;
        let n = to - from;
        let l2 = n.length_squared();
        if l2 < 1e-6 {
            return from;
        }
        let t = p.dot(n) / l2;
        if t <= 0.0 {
            from
        } else if t >= 1.0 {
            to
        } else {
            from + n * t
        }
    }

    /// Returns `true` if the polygon's vertices are ordered clockwise.
    ///
    /// _Godot equivalent: `is_polygon_clockwise`_
    pub fn is_polygon_clockwise(polygon: &[Vector2]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        let mut area = 0.0;
        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            area += (p2.x - p1.x) * (p2.y + p1.y);
        }
        area > 0.0
    }

    /// Returns `true` if `point` is inside `polygon`.
    ///
    /// _Godot equivalent: `is_point_in_polygon`_
    pub fn is_point_in_polygon(point: Vector2, polygon: &[Vector2]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = polygon.len() - 1;
        for i in 0..polygon.len() {
            if ((polygon[i].y > point.y) != (polygon[j].y > point.y))
                && (point.x < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                    / (polygon[j].y - polygon[i].y)
                    + polygon[i].x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_point_in_circle() {
        let center = Vector2::new(10.0, 10.0);
        let radius = 5.0;
        assert!(Geometry2D::is_point_in_circle(Vector2::new(10.0, 12.0), center, radius));
        assert!(Geometry2D::is_point_in_circle(Vector2::new(15.0, 10.0), center, radius));
        assert!(!Geometry2D::is_point_in_circle(Vector2::new(16.0, 10.0), center, radius));
    }

    #[test]
    fn test_is_point_in_triangle() {
        let a = Vector2::new(0.0, 0.0);
        let b = Vector2::new(10.0, 0.0);
        let c = Vector2::new(0.0, 10.0);
        assert!(Geometry2D::is_point_in_triangle(Vector2::new(2.0, 2.0), a, b, c));
        assert!(!Geometry2D::is_point_in_triangle(Vector2::new(10.0, 10.0), a, b, c));
    }

    #[test]
    fn test_segment_intersects_segment() {
        let a1 = Vector2::new(0.0, 0.0);
        let a2 = Vector2::new(10.0, 10.0);
        let b1 = Vector2::new(0.0, 10.0);
        let b2 = Vector2::new(10.0, 0.0);
        let intersection = Geometry2D::segment_intersects_segment(a1, a2, b1, b2);
        assert!(intersection.is_some());
        let p = intersection.unwrap();
        assert!((p.x - 5.0).abs() < 1e-6);
        assert!((p.y - 5.0).abs() < 1e-6);
    }
}

/// A library of static functions for 3D geometric operations.
///
/// Corresponds to Godot's `Geometry3D` singleton, but implemented in native Rust for performance.
pub struct Geometry3D;

impl Geometry3D {
    /// Returns the point on the segment (`from`, `to`) that is closest to `point`.
    ///
    /// _Godot equivalent: `get_closest_point_to_segment`_
    pub fn get_closest_point_to_segment(point: Vector3, from: Vector3, to: Vector3) -> Vector3 {
        let p = point - from;
        let n = to - from;
        let l2 = n.length_squared();
        if l2 < 1e-6 {
            return from;
        }
        let t = p.dot(n) / l2;
        if t <= 0.0 {
            from
        } else if t >= 1.0 {
            to
        } else {
            from + n * t
        }
    }

    /// Checks if the ray (`from`, `dir`) intersects the triangle (`a`, `b`, `c`).
    /// If it does, returns the intersection point.
    ///
    /// _Godot equivalent: `ray_intersects_triangle`_
    pub fn ray_intersects_triangle(from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Option<Vector3> {
        let edge1 = b - a;
        let edge2 = c - a;
        let h = dir.cross(edge2);
        let det = edge1.dot(h);

        if det > -1e-6 && det < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = from - a;
        let u = inv_det * s.dot(h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge1);
        let v = inv_det * dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * edge2.dot(q);

        if t > 1e-6 {
            Some(from + dir * t)
        } else {
            None
        }
    }

    /// Returns the 3D coordinates of the vertex `index` of the unit cube.
    ///
    /// _Godot equivalent: `get_cube_vertex`_
    pub fn get_cube_vertex(index: i32) -> Vector3 {
        Vector3::new(
            if (index & 1) != 0 { 1.0 } else { 0.0 },
            if (index & 2) != 0 { 1.0 } else { 0.0 },
            if (index & 4) != 0 { 1.0 } else { 0.0 },
        )
    }
}
