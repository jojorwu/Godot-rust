/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! RAII wrappers for `NavigationServer2D` and `NavigationServer3D` resources.

pub mod owned_agent_2d;
pub mod owned_agent_3d;
pub mod owned_link_2d;
pub mod owned_link_3d;
pub mod owned_map_2d;
pub mod owned_map_3d;
pub mod owned_obstacle_2d;
pub mod owned_obstacle_3d;
pub mod owned_region_2d;
pub mod owned_region_3d;

pub use owned_agent_2d::OwnedAgent2D;
pub use owned_agent_3d::OwnedAgent3D;
pub use owned_link_2d::OwnedLink2D;
pub use owned_link_3d::OwnedLink3D;
pub use owned_map_2d::OwnedMap2D;
pub use owned_map_3d::OwnedMap3D;
pub use owned_obstacle_2d::OwnedObstacle2D;
pub use owned_obstacle_3d::OwnedObstacle3D;
pub use owned_region_2d::OwnedRegion2D;
pub use owned_region_3d::OwnedRegion3D;

impl crate::classes::NavigationServer2D {
    /// Creates a new navigation map and returns a wrapper that will free it on drop.
    pub fn map_create_owned(&mut self) -> OwnedMap2D {
        OwnedMap2D::new()
    }

    /// Creates a new navigation region and returns a wrapper that will free it on drop.
    pub fn region_create_owned(&mut self) -> OwnedRegion2D {
        OwnedRegion2D::new()
    }

    /// Creates a new navigation link and returns a wrapper that will free it on drop.
    pub fn link_create_owned(&mut self) -> OwnedLink2D {
        OwnedLink2D::new()
    }

    /// Creates a new navigation agent and returns a wrapper that will free it on drop.
    pub fn agent_create_owned(&mut self) -> OwnedAgent2D {
        OwnedAgent2D::new()
    }

    /// Creates a new navigation obstacle and returns a wrapper that will free it on drop.
    pub fn obstacle_create_owned(&mut self) -> OwnedObstacle2D {
        OwnedObstacle2D::new()
    }
}

impl crate::classes::NavigationServer3D {
    /// Creates a new navigation map and returns a wrapper that will free it on drop.
    pub fn map_create_owned(&mut self) -> OwnedMap3D {
        OwnedMap3D::new()
    }

    /// Creates a new navigation region and returns a wrapper that will free it on drop.
    pub fn region_create_owned(&mut self) -> OwnedRegion3D {
        OwnedRegion3D::new()
    }

    /// Creates a new navigation link and returns a wrapper that will free it on drop.
    pub fn link_create_owned(&mut self) -> OwnedLink3D {
        OwnedLink3D::new()
    }

    /// Creates a new navigation agent and returns a wrapper that will free it on drop.
    pub fn agent_create_owned(&mut self) -> OwnedAgent3D {
        OwnedAgent3D::new()
    }

    /// Creates a new navigation obstacle and returns a wrapper that will free it on drop.
    pub fn obstacle_create_owned(&mut self) -> OwnedObstacle3D {
        OwnedObstacle3D::new()
    }
}
