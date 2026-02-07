/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
#[cfg(feature = "codegen-full")]
use godot::classes::Control;
use crate::framework::itest;

#[itest]
fn test_node_iter_children_typed() {
    let mut root = Node::new_alloc();

    let mut child1 = Node2D::new_alloc();
    child1.set_name("Child1");
    root.add_child(&child1);

    let mut child2 = Node::new_alloc();
    child2.set_name("Child2");
    root.add_child(&child2);

    let mut child3 = Node2D::new_alloc();
    child3.set_name("Child3");
    root.add_child(&child3);

    // Iter all as Node
    let all_children: Vec<Gd<Node>> = root.iter_children_typed::<Node>().collect();
    assert_eq!(all_children.len(), 3);

    // Iter only Node2D
    let node2d_children: Vec<Gd<Node2D>> = root.iter_children_typed::<Node2D>().collect();
    assert_eq!(node2d_children.len(), 2);
    assert_eq!(node2d_children[0].get_name(), StringName::from("Child1"));
    assert_eq!(node2d_children[1].get_name(), StringName::from("Child3"));

    // get_first_child_typed
    let first_node2d = root.get_first_child_typed::<Node2D>();
    assert!(first_node2d.is_some());
    assert_eq!(first_node2d.unwrap().get_name(), StringName::from("Child1"));

    #[cfg(feature = "codegen-full")]
    {
        let no_control = root.get_first_child_typed::<Control>();
        assert!(no_control.is_none());
    }

    root.free();
}
