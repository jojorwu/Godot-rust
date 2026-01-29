/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::builtin::inner::InnerRid;
use godot::builtin::Rid;
use godot::classes::RenderingServer;
use godot::obj::{IndexEnum, Singleton};
use godot::prelude::*;

use crate::framework::{itest, suppress_godot_print};

#[itest]
fn rid_equiv() {
    let invalid: Rid = Rid::Invalid;
    let valid: Rid = Rid::new((10 << 32) | 20);
    assert!(!InnerRid::from_outer(&invalid).is_valid());
    assert!(InnerRid::from_outer(&valid).is_valid());

    assert_eq!(InnerRid::from_outer(&invalid).get_id(), 0);
    assert_eq!(InnerRid::from_outer(&valid).get_id(), (10 << 32) | 20);
}

#[itest]
fn canvas_set_parent() {
    // This originally caused UB, but still testing it here in case it breaks.
    let mut server = RenderingServer::singleton();
    let canvas = server.canvas_create_owned();
    let viewport = server.viewport_create_owned();

    suppress_godot_print(|| server.canvas_item_set_parent(*viewport, *canvas));
    suppress_godot_print(|| server.canvas_item_set_parent(*viewport, *viewport));
}

#[itest]
#[cfg(feature = "experimental-threads")]
fn multi_thread_test() {
    use std::collections::HashSet;

    use godot::builtin::{Color, Vector2};

    use godot::rendering::OwnedCanvasItem;

    let threads = (0..10)
        .map(|_| {
            std::thread::spawn(|| {
                let mut server = RenderingServer::singleton();
                (0..1000)
                    .map(|_| server.canvas_item_create_owned())
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    let mut items: Vec<OwnedCanvasItem> = vec![];

    for thread in threads.into_iter() {
        items.append(&mut thread.join().unwrap());
    }

    let set = items.iter().map(|i| i.rid()).collect::<HashSet<_>>();
    assert_eq!(set.len(), items.len());

    let mut server = RenderingServer::singleton();

    for item in items.iter() {
        server.canvas_item_add_circle(**item, Vector2::ZERO, 1.0, Color::from_rgb(1.0, 0.0, 0.0));
    }
}

/// Check that godot does not crash upon receiving various RIDs that may be edge cases. As it could do in Godot 3.
#[itest]
fn strange_rids() {
    let mut server = RenderingServer::singleton();
    let mut rids: Vec<u64> = vec![
        // Invalid RID.
        0,
        // Normal RID, should work without issue.
        1,
        10,
        // Testing the boundaries of various ints.
        u8::MAX as u64,
        u16::MAX as u64,
        u32::MAX as u64,
        u64::MAX,
        i8::MIN as u64,
        i8::MAX as u64,
        i16::MIN as u64,
        i16::MAX as u64,
        i32::MIN as u64,
        i32::MAX as u64,
        i64::MIN as u64,
        i64::MAX as u64,
        // Biggest RIDs possible in Godot (ignoring local indices).
        0xFFFFFFFF << 32,
        0x7FFFFFFF << 32,
        // Godot's servers treats RIDs as two u32s, so testing what happens round the region where
        // one u32 overflows into the next.
        u32::MAX as u64 + 1,
        u32::MAX as u64 + 2,
        u32::MAX as u64 - 1,
        u32::MAX as u64 - 2,
        // A couple random RIDs.
        1234567891011121314,
        14930753991246632225,
        8079365198791785081,
        10737267678893224303,
        12442588258967011829,
        4275912429544145425,
    ];
    // Checking every number with exactly 2 bits = 1.
    // An approximation of exhaustively checking every number.
    for i in 0..64 {
        for j in 0..63 {
            if j >= i {
                rids.push((1 << i) | (1 << (j + 1)))
            } else {
                rids.push((1 << i) | (1 << j))
            }
        }
    }

    for id in rids.iter() {
        suppress_godot_print(|| server.canvas_item_clear(Rid::new(*id)))
    }
}

#[itest]
fn owned_texture_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let texture = server.texture_2d_placeholder_create_owned();
        let rid = texture.rid();
        assert!(rid.is_valid());
        rid
    };

    // After the texture is dropped, the RID should be freed and reused.
    let texture2 = server.texture_2d_placeholder_create_owned();
    assert_eq!(rid, texture2.rid());
}

#[itest]
fn owned_material_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let mut material = server.material_create_owned();
        let rid = *material;
        assert!(rid.is_valid());

        material.set_param("diffuse_mode", &3_i32.to_variant()); // Lambert
        material.set_param("specular", &0.5_f32.to_variant());

        rid
    };

    // After the material is dropped, the RID should be freed and reused.
    let material2 = server.material_create_owned();
    assert_eq!(rid, *material2);
}

#[itest]
fn owned_mesh_raii() {
    use godot::classes::mesh::ArrayType;
    use godot::classes::rendering_server::PrimitiveType;

    let mut server = RenderingServer::singleton();
    let rid = {
        let mut mesh = server.mesh_create_owned();
        let rid = *mesh;
        assert!(rid.is_valid());

        let mut arrays = VarArray::new();
        arrays.resize(ArrayType::MAX.to_index(), &Variant::nil());

        // Add a single triangle
        let vertices = PackedVector3Array::from_iter([
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ]);
        arrays.set(ArrayType::VERTEX.to_index(), &vertices.to_variant());

        mesh.add_surface(PrimitiveType::TRIANGLES, &arrays);

        assert_eq!(mesh.surface_get_array_len(0), 3);

        mesh.clear();
        assert_eq!(mesh.surface_get_array_len(0), 0);

        rid
    };

    // After the mesh is dropped, the RID should be freed and reused.
    let mesh2 = server.mesh_create_owned();
    assert_eq!(rid, *mesh2);
}

#[itest]
fn owned_shader_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let mut shader = server.shader_create_owned();
        let rid = *shader;
        assert!(rid.is_valid());

        let code = "shader_type spatial; void fragment() { ALBEDO = vec3(1.0, 0.0, 0.0); }";
        shader.set_code(code);

        rid
    };

    // After the shader is dropped, the RID should be freed and reused.
    let shader2 = server.shader_create_owned();
    assert_eq!(rid, *shader2);
}

#[itest]
fn owned_light_raii() {
    use godot::builtin::Color;
    use godot::classes::rendering_server::LightType;

    let mut server = RenderingServer::singleton();
    let rid = {
        let mut light = server.light_create_owned(LightType::DIRECTIONAL);
        let rid = *light;
        assert!(rid.is_valid());

        light.set_color(Color::from_rgb(1.0, 0.5, 0.0));

        rid
    };

    // After the light is dropped, the RID should be freed and reused.
    let light2 = server.light_create_owned(LightType::OMNI);
    assert_eq!(rid, *light2);
}

#[itest]
fn owned_viewport_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let mut viewport = server.viewport_create_owned();
        let rid = *viewport;
        assert!(rid.is_valid());

        viewport.set_size(128, 64);

        rid
    };

    // After the viewport is dropped, the RID should be freed and reused.
    let viewport2 = server.viewport_create_owned();
    assert_eq!(rid, *viewport2);
}

#[itest]
fn owned_canvas_item_raii() {
    let mut server = RenderingServer::singleton();
    let parent = server.viewport_create_owned();

    let rid = {
        let mut item = server.canvas_item_create_owned();
        let rid = *item;
        assert!(rid.is_valid());

        item.set_parent(*parent);
        item.add_circle(
            Vector2::new(10.0, 20.0),
            5.0,
            Color::from_rgb(1.0, 0.0, 0.0),
        );
        item.set_modulate(Color::from_rgb(0.5, 0.5, 0.5));
        item.set_transform(&Transform2D::from_angle_origin(
            0.0,
            Vector2::new(50.0, 50.0),
        ));

        rid
    };

    // After the item is dropped, the RID should be freed and reused.
    let item2 = server.canvas_item_create_owned();
    assert_eq!(rid, *item2);
}

#[itest]
fn owned_canvas_raii() {
    let mut server = RenderingServer::singleton();
    let item = server.canvas_item_create_owned();

    let rid = {
        let mut canvas = server.canvas_create_owned();
        let rid = *canvas;
        assert!(rid.is_valid());

        canvas.set_item_mirroring(*item, Vector2::new(1.0, 0.0));

        rid
    };

    // After the canvas is dropped, the RID should be freed and reused.
    let canvas2 = server.canvas_create_owned();
    assert_eq!(rid, *canvas2);
}

#[itest]
fn owned_camera_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let camera = server.camera_create_owned();
        let rid = *camera;
        assert!(rid.is_valid());
        rid
    };

    let camera2 = server.camera_create_owned();
    assert_eq!(rid, *camera2);
}

#[itest]
fn owned_scenario_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let scenario = server.scenario_create_owned();
        let rid = *scenario;
        assert!(rid.is_valid());
        rid
    };

    let scenario2 = server.scenario_create_owned();
    assert_eq!(rid, *scenario2);
}

#[itest]
fn owned_instance_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let instance = server.instance_create_owned();
        let rid = *instance;
        assert!(rid.is_valid());
        rid
    };

    let instance2 = server.instance_create_owned();
    assert_eq!(rid, *instance2);
}

#[itest]
fn owned_environment_raii() {
    let mut server = RenderingServer::singleton();
    let rid = {
        let environment = server.environment_create_owned();
        let rid = *environment;
        assert!(rid.is_valid());
        rid
    };

    let environment2 = server.environment_create_owned();
    assert_eq!(rid, *environment2);
}

#[itest]
#[cfg(feature = "codegen-full")]
fn owned_physics_2d_raii() {
    use godot::classes::physics_server_2d::ShapeType;
    use godot::classes::PhysicsServer2D;

    let mut server = PhysicsServer2D::singleton();

    let rid_space = {
        let space = server.space_create_owned();
        let rid = *space;
        assert!(rid.is_valid());
        rid
    };
    let space2 = server.space_create_owned();
    assert_eq!(rid_space, *space2);

    let rid_body = {
        let body = server.body_create_owned();
        let rid = *body;
        assert!(rid.is_valid());
        rid
    };
    let body2 = server.body_create_owned();
    assert_eq!(rid_body, *body2);

    let rid_area = {
        let area = server.area_create_owned();
        let rid = *area;
        assert!(rid.is_valid());
        rid
    };
    let area2 = server.area_create_owned();
    assert_eq!(rid_area, *area2);

    let rid_shape = {
        let shape = server.shape_create_owned(ShapeType::CIRCLE);
        let rid = *shape;
        assert!(rid.is_valid());
        rid
    };
    let shape2 = server.shape_create_owned(ShapeType::CIRCLE);
    assert_eq!(rid_shape, *shape2);
}

#[itest]
#[cfg(feature = "codegen-full")]
fn owned_physics_3d_raii() {
    use godot::classes::physics_server_3d::ShapeType;
    use godot::classes::PhysicsServer3D;

    let mut server = PhysicsServer3D::singleton();

    let rid_space = {
        let space = server.space_create_owned();
        let rid = *space;
        assert!(rid.is_valid());
        rid
    };
    let space2 = server.space_create_owned();
    assert_eq!(rid_space, *space2);

    let rid_body = {
        let body = server.body_create_owned();
        let rid = *body;
        assert!(rid.is_valid());
        rid
    };
    let body2 = server.body_create_owned();
    assert_eq!(rid_body, *body2);

    let rid_soft_body = {
        let soft_body = server.soft_body_create_owned();
        let rid = *soft_body;
        assert!(rid.is_valid());
        rid
    };
    let soft_body2 = server.soft_body_create_owned();
    assert_eq!(rid_soft_body, *soft_body2);

    let rid_area = {
        let area = server.area_create_owned();
        let rid = *area;
        assert!(rid.is_valid());
        rid
    };
    let area2 = server.area_create_owned();
    assert_eq!(rid_area, *area2);

    let rid_shape = {
        let shape = server.shape_create_owned(ShapeType::BOX);
        let rid = *shape;
        assert!(rid.is_valid());
        rid
    };
    let shape2 = server.shape_create_owned(ShapeType::BOX);
    assert_eq!(rid_shape, *shape2);
}

#[itest]
#[cfg(feature = "codegen-full-experimental")]
fn owned_navigation_2d_raii() {
    use godot::classes::NavigationServer2D;

    let mut server = NavigationServer2D::singleton();

    let rid_map = {
        let map = server.map_create_owned();
        let rid = *map;
        assert!(rid.is_valid());
        rid
    };
    let map2 = server.map_create_owned();
    assert_eq!(rid_map, *map2);

    let rid_region = {
        let region = server.region_create_owned();
        let rid = *region;
        assert!(rid.is_valid());
        rid
    };
    let region2 = server.region_create_owned();
    assert_eq!(rid_region, *region2);
}

#[itest]
#[cfg(feature = "codegen-full-experimental")]
fn owned_navigation_3d_raii() {
    use godot::classes::NavigationServer3D;

    let mut server = NavigationServer3D::singleton();

    let rid_map = {
        let map = server.map_create_owned();
        let rid = *map;
        assert!(rid.is_valid());
        rid
    };
    let map2 = server.map_create_owned();
    assert_eq!(rid_map, *map2);

    let rid_region = {
        let region = server.region_create_owned();
        let rid = *region;
        assert!(rid.is_valid());
        rid
    };
    let region2 = server.region_create_owned();
    assert_eq!(rid_region, *region2);
}

#[itest]
#[cfg(feature = "codegen-full")]
fn owned_text_server_raii() {
    use godot::classes::TextServerManager;

    let tsm = TextServerManager::singleton();
    let mut ts = tsm.get_primary_interface().expect("primary text server");

    let rid_font = {
        let font = ts.create_font_owned();
        let rid = *font;
        assert!(rid.is_valid());
        rid
    };
    let font2 = ts.create_font_owned();
    assert_eq!(rid_font, *font2);

    let rid_shaped = {
        let shaped = ts.create_shaped_text_owned();
        let rid = *shaped;
        assert!(rid.is_valid());
        rid
    };
    let shaped2 = ts.create_shaped_text_owned();
    assert_eq!(rid_shaped, *shaped2);
}

#[itest]
#[cfg(feature = "codegen-full")]
fn owned_rendering_device_raii() {
    use godot::classes::RenderingServer;

    let rs = RenderingServer::singleton();
    let rd = rs.get_rendering_device();

    // RenderingDevice might be null if using Compatibility renderer or on unsupported platforms.
    let Some(mut rd) = rd else {
        return;
    };

    let rid_buffer = {
        let buffer = rd.storage_buffer_create_owned(1024);
        let rid = *buffer;
        assert!(rid.is_valid());
        rid
    };

    // After the buffer is dropped, the RID should be freed and reused.
    let buffer2 = rd.storage_buffer_create_owned(1024);
    assert_eq!(rid_buffer, *buffer2);
}
