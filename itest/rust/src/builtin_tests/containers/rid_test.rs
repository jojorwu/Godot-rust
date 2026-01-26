/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::builtin::inner::InnerRid;
use godot::builtin::Rid;
use godot::classes::RenderingServer;
use godot::obj::Singleton;
use godot::rendering::owned_light::OwnedLight;
use godot::rendering::owned_material::OwnedMaterial;
use godot::rendering::owned_mesh::OwnedMesh;
use godot::rendering::owned_shader::OwnedShader;
use godot::rendering::owned_texture::OwnedTexture;
use godot::rendering::owned_viewport::OwnedViewport;

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
    let canvas = server.canvas_create();
    let viewport = server.viewport_create();

    suppress_godot_print(|| server.canvas_item_set_parent(viewport, canvas));
    suppress_godot_print(|| server.canvas_item_set_parent(viewport, viewport));

    server.free_rid(canvas);
    server.free_rid(viewport);
}

#[itest]
#[cfg(feature = "experimental-threads")]
fn multi_thread_test() {
    use std::collections::HashSet;

    use godot::builtin::{Color, Vector2};

    let threads = (0..10)
        .map(|_| {
            std::thread::spawn(|| {
                let mut server = RenderingServer::singleton();
                (0..1000).map(|_| server.canvas_item_create()).collect()
            })
        })
        .collect::<Vec<_>>();

    let mut rids: Vec<Rid> = vec![];

    for thread in threads.into_iter() {
        rids.append(&mut thread.join().unwrap());
    }

    let set = rids.iter().cloned().collect::<HashSet<_>>();
    assert_eq!(set.len(), rids.len());

    let mut server = RenderingServer::singleton();

    for rid in rids.iter() {
        server.canvas_item_add_circle(*rid, Vector2::ZERO, 1.0, Color::from_rgb(1.0, 0.0, 0.0));
    }

    for rid in rids.iter() {
        server.free_rid(*rid);
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
    let rid = {
        let texture = OwnedTexture::new_placeholder();
        let rid = texture.rid();
        assert!(rid.is_valid());
        rid
    };

    // After the texture is dropped, the RID should be freed and reused.
    let texture2 = OwnedTexture::new_placeholder();
    assert_eq!(rid, texture2.rid());
}

#[itest]
fn owned_material_raii() {
    let rid = {
        let mut material = OwnedMaterial::new();
        let rid = material.rid();
        assert!(rid.is_valid());

        material.set_param("diffuse_mode", &3.to_variant()); // Lambert
        material.set_param("specular", &0.5.to_variant());

        rid
    };

    // After the material is dropped, the RID should be freed and reused.
    let material2 = OwnedMaterial::new();
    assert_eq!(rid, material2.rid());
}

#[itest]
fn owned_mesh_raii() {
    use godot::builtin::Dictionary;
    use godot::classes::rendering_server::PrimitiveType;

    let rid = {
        let mut mesh = OwnedMesh::new();
        let rid = mesh.rid();
        assert!(rid.is_valid());

        let mut arrays = Dictionary::new();
        // Add a single triangle
        arrays.set("vertex", vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0].to_variant());
        mesh.add_surface(PrimitiveType::TRIANGLES, &arrays);

        rid
    };

    // After the mesh is dropped, the RID should be freed and reused.
    let mesh2 = OwnedMesh::new();
    assert_eq!(rid, mesh2.rid());
}

#[itest]
fn owned_shader_raii() {
    let rid = {
        let mut shader = OwnedShader::new();
        let rid = shader.rid();
        assert!(rid.is_valid());

        let code = "shader_type spatial; void fragment() { ALBEDO = vec3(1.0, 0.0, 0.0); }";
        shader.set_code(code);

        rid
    };

    // After the shader is dropped, the RID should be freed and reused.
    let shader2 = OwnedShader::new();
    assert_eq!(rid, shader2.rid());
}

#[itest]
fn owned_light_raii() {
    use godot::builtin::Color;
    use godot::classes::rendering_server::LightType;

    let rid = {
        let mut light = OwnedLight::new(LightType::DIRECTIONAL);
        let rid = light.rid();
        assert!(rid.is_valid());

        light.set_color(Color::from_rgb(1.0, 0.5, 0.0));

        rid
    };

    // After the light is dropped, the RID should be freed and reused.
    let light2 = OwnedLight::new(LightType::OMNI);
    assert_eq!(rid, light2.rid());
}

#[itest]
fn owned_viewport_raii() {
    let rid = {
        let mut viewport = OwnedViewport::new();
        let rid = viewport.rid();
        assert!(rid.is_valid());

        viewport.set_size(128, 64);

        rid
    };

    // After the viewport is dropped, the RID should be freed and reused.
    let viewport2 = OwnedViewport::new();
    assert_eq!(rid, viewport2.rid());
}
