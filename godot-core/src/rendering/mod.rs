/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `RenderingServer` resources.

macro_rules! impl_owned_rid {
    ($name:ident, $doc:literal) => {
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

        impl Drop for $name {
            fn drop(&mut self) {
                if self.rid.is_valid() {
                    use crate::obj::Singleton as _;
                    crate::classes::RenderingServer::singleton().free_rid(self.rid);
                }
            }
        }
    };
}

pub(crate) use impl_owned_rid;

pub mod owned_camera;
pub mod owned_canvas;
pub mod owned_canvas_item;
pub mod owned_environment;
pub mod owned_instance;
pub mod owned_light;
pub mod owned_material;
pub mod owned_mesh;
pub mod owned_scenario;
pub mod owned_shader;
pub mod owned_texture;
pub mod owned_viewport;

pub use owned_camera::OwnedCamera;
pub use owned_canvas::OwnedCanvas;
pub use owned_canvas_item::OwnedCanvasItem;
pub use owned_environment::OwnedEnvironment;
pub use owned_instance::OwnedInstance;
pub use owned_light::OwnedLight;
pub use owned_material::OwnedMaterial;
pub use owned_mesh::OwnedMesh;
pub use owned_scenario::OwnedScenario;
pub use owned_shader::OwnedShader;
pub use owned_texture::OwnedTexture;
pub use owned_viewport::OwnedViewport;

impl crate::classes::RenderingServer {
    /// Creates a new camera and returns a wrapper that will free it on drop.
    pub fn camera_create_owned(&mut self) -> OwnedCamera {
        OwnedCamera::new()
    }

    /// Creates a new canvas and returns a wrapper that will free it on drop.
    pub fn canvas_create_owned(&mut self) -> OwnedCanvas {
        OwnedCanvas::new()
    }

    /// Creates a new canvas item and returns a wrapper that will free it on drop.
    pub fn canvas_item_create_owned(&mut self) -> OwnedCanvasItem {
        OwnedCanvasItem::new()
    }

    /// Creates a new environment and returns a wrapper that will free it on drop.
    pub fn environment_create_owned(&mut self) -> OwnedEnvironment {
        OwnedEnvironment::new()
    }

    /// Creates a new instance and returns a wrapper that will free it on drop.
    pub fn instance_create_owned(&mut self) -> OwnedInstance {
        OwnedInstance::new()
    }

    /// Creates a new light and returns a wrapper that will free it on drop.
    pub fn light_create_owned(
        &mut self,
        light_type: crate::classes::rendering_server::LightType,
    ) -> OwnedLight {
        OwnedLight::new(light_type)
    }

    /// Creates a new material and returns a wrapper that will free it on drop.
    pub fn material_create_owned(&mut self) -> OwnedMaterial {
        OwnedMaterial::new()
    }

    /// Creates a new mesh and returns a wrapper that will free it on drop.
    pub fn mesh_create_owned(&mut self) -> OwnedMesh {
        OwnedMesh::new()
    }

    /// Creates a new scenario and returns a wrapper that will free it on drop.
    pub fn scenario_create_owned(&mut self) -> OwnedScenario {
        OwnedScenario::new()
    }

    /// Creates a new shader and returns a wrapper that will free it on drop.
    pub fn shader_create_owned(&mut self) -> OwnedShader {
        OwnedShader::new()
    }

    /// Creates a new texture and returns a wrapper that will free it on drop.
    #[cfg(feature = "codegen-full")]
    pub fn texture_2d_create_owned(
        &mut self,
        image: &crate::obj::Gd<crate::classes::Image>,
    ) -> OwnedTexture {
        OwnedTexture::new(image)
    }

    /// Creates a new placeholder texture and returns a wrapper that will free it on drop.
    pub fn texture_2d_placeholder_create_owned(&mut self) -> OwnedTexture {
        OwnedTexture::new_placeholder()
    }

    /// Creates a new viewport and returns a wrapper that will free it on drop.
    pub fn viewport_create_owned(&mut self) -> OwnedViewport {
        OwnedViewport::new()
    }
}
