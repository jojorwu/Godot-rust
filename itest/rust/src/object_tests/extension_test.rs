/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use godot::classes::{Node, Resource};
use crate::framework::itest;

#[itest]
fn test_object_property_accessors() {
    let mut node = Node::new_alloc();
    let name = "TestNode";

    // Test set_as and get_as
    node.set_as("name", name);
    let name_back: GString = node.get_as("name");
    assert_eq!(name_back, GString::from(name));

    // Test try_get_as
    let name_opt: Option<GString> = node.try_get_as("name");
    assert_eq!(name_opt, Some(GString::from(name)));

    let bad_opt: Option<i64> = node.try_get_as("name");
    assert_eq!(bad_opt, None);

    node.free();
}

#[itest]
fn test_metadata_dictionary_conversions() {
    use godot::meta::{PropertyInfo, MethodInfo, ClassId};
    use godot::builtin::VariantType;
    use godot::global::PropertyUsageFlags;

    // PropertyInfo round-trip
    let prop = PropertyInfo {
        variant_type: VariantType::INT,
        class_id: ClassId::none(),
        property_name: StringName::from("test_prop"),
        hint_info: godot::meta::PropertyHintInfo::none(),
        usage: PropertyUsageFlags::DEFAULT,
    };

    let dict = prop.to_dictionary();
    let prop_back = PropertyInfo::from_dictionary(&dict);

    assert_eq!(prop_back.property_name, prop.property_name);
    assert_eq!(prop_back.variant_type, prop.variant_type);
    assert_eq!(prop_back.usage, prop.usage);

    // MethodInfo round-trip
    let method = MethodInfo::new("test_method")
        .with_id(42)
        .with_return_type(PropertyInfo::new_var::<i64>(""))
        .with_argument(PropertyInfo::new_var::<GString>("arg1"));

    let m_dict = method.to_dictionary();
    let method_back = MethodInfo::from_dictionary(&m_dict);

    assert_eq!(method_back.method_name, method.method_name);
    assert_eq!(method_back.id, method.id);
    assert_eq!(method_back.arguments.len(), method.arguments.len());
    assert_eq!(method_back.arguments[0].property_name, method.arguments[0].property_name);
}

#[itest]
fn test_object_meta_accessors() {
    let mut node = Node::new_alloc();
    let meta_key = "my_meta";
    let meta_val = 42i64;

    // Test set_meta_as and get_meta_as
    node.set_meta_as(meta_key, meta_val);
    let val_back: i64 = node.get_meta_as(meta_key);
    assert_eq!(val_back, meta_val);

    // Test try_get_meta_as
    let val_opt: Option<i64> = node.try_get_meta_as(meta_key);
    assert_eq!(val_opt, Some(meta_val));

    let bad_opt: Option<GString> = node.try_get_meta_as(meta_key);
    assert_eq!(bad_opt, None);

    node.free();
}

#[itest]
fn test_resource_duplication() {
    let res = Resource::new_gd();

    // Test duplicate_as
    let dup: Gd<Resource> = res.duplicate_as(false);
    assert!(dup.is_instance_valid());
    assert_ne!(res.instance_id(), dup.instance_id());

    // Test duplicate_typed
    let dup_typed: Gd<Resource> = res.duplicate_typed::<Resource>(false);
    assert!(dup_typed.is_instance_valid());
}

#[itest]
fn test_object_call_as() {
    let mut node = Node::new_alloc();

    // Test call_as
    let name: GString = node.call_as("get_name", &[]);
    assert!(!name.is_empty());

    // Test try_call_as
    let name_opt: Option<GString> = node.try_call_as("get_name", &[]);
    assert!(name_opt.is_some());

    let bad_opt: Option<i64> = node.try_call_as("get_name", &[]);
    assert_eq!(bad_opt, None);

    node.free();
}

#[itest]
fn test_node_find_child_typed() {
    let mut parent = Node::new_alloc();
    let mut child = Node::new_alloc();
    let child_name = "MyChild";
    child.set_name(child_name);
    parent.add_child(&child);

    let found = parent.find_child_typed::<Node>(child_name, false, true);
    assert!(found.is_some());
    assert_eq!(found.unwrap().instance_id(), child.instance_id());

    parent.remove_child(&child);
    child.free();
    parent.free();
}

#[itest]
fn test_object_get_property_list_typed() {
    let node = Node::new_alloc();
    let props = node.get_property_list_typed();

    assert!(!props.is_empty());
    assert!(props.iter().any(|p| p.property_name == StringName::from("name")));

    node.free();
}
