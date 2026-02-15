/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `RenderingDevice` resources.

pub mod helper;
pub mod owned_rd_buffer;
pub mod owned_rd_framebuffer;
pub mod owned_rd_index_array;
pub mod owned_rd_pipeline;
pub mod owned_rd_sampler;
pub mod owned_rd_shader;
pub mod owned_rd_texture;
pub mod owned_rd_uniform_set;
pub mod owned_rd_vertex_array;

pub use owned_rd_buffer::OwnedRdBuffer;
pub use owned_rd_framebuffer::OwnedRdFramebuffer;
pub use owned_rd_index_array::OwnedRdIndexArray;
pub use owned_rd_pipeline::OwnedRdPipeline;
pub use owned_rd_sampler::OwnedRdSampler;
pub use owned_rd_shader::OwnedRdShader;
pub use owned_rd_texture::OwnedRdTexture;
pub use owned_rd_uniform_set::OwnedRdUniformSet;
pub use owned_rd_vertex_array::OwnedRdVertexArray;

impl crate::classes::RenderingDevice {
    /// Creates a new texture and returns a wrapper that will free it on drop.
    pub fn texture_create_owned(
        &mut self,
        format: impl crate::meta::AsArg<Option<crate::obj::Gd<crate::classes::RdTextureFormat>>>,
        view: impl crate::meta::AsArg<Option<crate::obj::Gd<crate::classes::RdTextureView>>>,
    ) -> OwnedRdTexture {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.texture_create(format, view);
        unsafe { OwnedRdTexture::from_rid(rid, gd) }
    }

    /// Creates a new sampler and returns a wrapper that will free it on drop.
    pub fn sampler_create_owned(
        &mut self,
        state: impl crate::meta::AsArg<Option<crate::obj::Gd<crate::classes::RdSamplerState>>>,
    ) -> OwnedRdSampler {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.sampler_create(state);
        unsafe { OwnedRdSampler::from_rid(rid, gd) }
    }

    /// Creates a new shader from SPIR-V and returns a wrapper that will free it on drop.
    pub fn shader_create_from_spirv_owned(
        &mut self,
        spirv_data: impl crate::meta::AsArg<Option<crate::obj::Gd<crate::classes::RdShaderSpirv>>>,
    ) -> OwnedRdShader {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.shader_create_from_spirv(spirv_data);
        unsafe { OwnedRdShader::from_rid(rid, gd) }
    }

    /// Creates a new uniform buffer and returns a wrapper that will free it on drop.
    pub fn uniform_buffer_create_owned(&mut self, size_bytes: u32) -> OwnedRdBuffer {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.uniform_buffer_create(size_bytes);
        unsafe { OwnedRdBuffer::from_rid(rid, gd) }
    }

    /// Creates a new storage buffer and returns a wrapper that will free it on drop.
    pub fn storage_buffer_create_owned(&mut self, size_bytes: u32) -> OwnedRdBuffer {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.storage_buffer_create(size_bytes);
        unsafe { OwnedRdBuffer::from_rid(rid, gd) }
    }

    /// Creates a new vertex buffer and returns a wrapper that will free it on drop.
    pub fn vertex_buffer_create_owned(&mut self, size_bytes: u32) -> OwnedRdBuffer {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.vertex_buffer_create(size_bytes);
        unsafe { OwnedRdBuffer::from_rid(rid, gd) }
    }

    /// Creates a new index buffer and returns a wrapper that will free it on drop.
    pub fn index_buffer_create_owned(
        &mut self,
        size_indices: u32,
        format: crate::classes::rendering_device::IndexBufferFormat,
    ) -> OwnedRdBuffer {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.index_buffer_create(size_indices, format);
        unsafe { OwnedRdBuffer::from_rid(rid, gd) }
    }

    /// Creates a new vertex array and returns a wrapper that will free it on drop.
    pub fn vertex_array_create_owned(
        &mut self,
        vertex_count: u32,
        vertex_format: i64,
        src_buffers: &crate::builtin::Array<crate::builtin::Rid>,
    ) -> OwnedRdVertexArray {
        let gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        OwnedRdVertexArray::new(&gd, vertex_count, vertex_format, src_buffers)
    }

    /// Creates a new index array and returns a wrapper that will free it on drop.
    pub fn index_array_create_owned(
        &mut self,
        index_buffer: crate::builtin::Rid,
        index_offset: u32,
        index_count: u32,
    ) -> OwnedRdIndexArray {
        let gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        OwnedRdIndexArray::new(&gd, index_buffer, index_offset, index_count)
    }

    /// Creates a new framebuffer and returns a wrapper that will free it on drop.
    pub fn framebuffer_create_owned(
        &mut self,
        textures: &crate::builtin::Array<crate::builtin::Rid>,
    ) -> OwnedRdFramebuffer {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.framebuffer_create(textures);
        unsafe { OwnedRdFramebuffer::from_rid(rid, gd) }
    }

    /// Creates a new uniform set and returns a wrapper that will free it on drop.
    pub fn uniform_set_create_owned(
        &mut self,
        uniforms: &crate::builtin::Array<crate::obj::Gd<crate::classes::RdUniform>>,
        shader: crate::builtin::Rid,
        shader_set: u32,
    ) -> OwnedRdUniformSet {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.uniform_set_create(uniforms, shader, shader_set);
        unsafe { OwnedRdUniformSet::from_rid(rid, gd) }
    }

    /// Creates a new compute pipeline and returns a wrapper that will free it on drop.
    pub fn compute_pipeline_create_owned(
        &mut self,
        shader: crate::builtin::Rid,
    ) -> OwnedRdPipeline {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.compute_pipeline_create(shader);
        unsafe { OwnedRdPipeline::from_rid(rid, gd) }
    }

    /// Creates a new render pipeline and returns a wrapper that will free it on drop.
    #[allow(clippy::too_many_arguments)]
    pub fn render_pipeline_create_owned(
        &mut self,
        shader: crate::builtin::Rid,
        framebuffer_format: i64,
        vertex_format: i64,
        primitive: crate::classes::rendering_device::RenderPrimitive,
        rasterization_state: impl crate::meta::AsArg<
            Option<crate::obj::Gd<crate::classes::RdPipelineRasterizationState>>,
        >,
        multisample_state: impl crate::meta::AsArg<
            Option<crate::obj::Gd<crate::classes::RdPipelineMultisampleState>>,
        >,
        stencil_state: impl crate::meta::AsArg<
            Option<crate::obj::Gd<crate::classes::RdPipelineDepthStencilState>>,
        >,
        color_blend_state: impl crate::meta::AsArg<
            Option<crate::obj::Gd<crate::classes::RdPipelineColorBlendState>>,
        >,
    ) -> OwnedRdPipeline {
        let mut gd = crate::private::rebuild_gd(self).cast::<crate::classes::RenderingDevice>();
        let rid = gd.render_pipeline_create(
            shader,
            framebuffer_format,
            vertex_format,
            primitive,
            rasterization_state,
            multisample_state,
            stencil_state,
            color_blend_state,
        );
        unsafe { OwnedRdPipeline::from_rid(rid, gd) }
    }

    /// Frees the resource with the given RID.
    ///
    /// See `RenderingDevice.free_rid()`.
    pub fn free_rid_owned(&mut self, rid: crate::builtin::Rid) {
        self.free_rid(rid);
    }
}
