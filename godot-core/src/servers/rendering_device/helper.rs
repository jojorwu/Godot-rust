/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::builtin::Rid;
use crate::classes::RenderingDevice;
use crate::obj::Gd;
use crate::servers::rendering_device::{OwnedRdPipeline, OwnedRdUniformSet};

/// A helper for simplified compute shader dispatch.
pub struct ComputePipeline {
    rd: Gd<RenderingDevice>,
    pipeline: OwnedRdPipeline,
    uniform_sets: Vec<OwnedRdUniformSet>,
}

impl ComputePipeline {
    /// Creates a new compute pipeline for the given shader.
    pub fn new(mut rd: Gd<RenderingDevice>, shader: Rid) -> Self {
        let pipeline = rd.compute_pipeline_create_owned(shader);
        Self {
            rd,
            pipeline,
            uniform_sets: Vec::new(),
        }
    }

    /// Binds a uniform set to the pipeline.
    pub fn bind_uniform_set(&mut self, uniform_set: OwnedRdUniformSet) {
        self.uniform_sets.push(uniform_set);
    }

    /// Dispatches the compute shader.
    pub fn dispatch(&mut self, x_groups: u32, y_groups: u32, z_groups: u32) {
        let compute_list = self.rd.compute_list_begin();
        self.rd.compute_list_bind_compute_pipeline(compute_list, self.pipeline.rid());

        for (i, uniform_set) in self.uniform_sets.iter().enumerate() {
            self.rd.compute_list_bind_uniform_set(compute_list, uniform_set.rid(), i as u32);
        }

        self.rd.compute_list_dispatch(compute_list, x_groups, y_groups, z_groups);
        self.rd.compute_list_end();
    }

    /// Submits the compute work and optionally waits for it to finish.
    pub fn submit(&mut self, wait: bool) {
        self.rd.submit();
        if wait {
            self.rd.sync();
        }
    }
}
