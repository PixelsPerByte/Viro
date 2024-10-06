use std::any::TypeId;

use bevy::{ecs::component::ComponentInfo, prelude::*};
use bevy_egui::egui;

use crate::{interface::components::ComponentUis, SelectedEntities};

pub fn show(world: &mut World, ui: &mut egui::Ui) {
    let selected_entities = world.resource::<SelectedEntities>();

    let Some(entity) = selected_entities.0.iter().next().cloned() else {
        return;
    };

    world.resource_scope::<ComponentUis, _>(|world, component_uis| {
        let components: Vec<ComponentInfo> =
            world.inspect_entity(entity).into_iter().cloned().collect();

        for info in components {
            if is_hidden_component(info.type_id()) {
                continue;
            }

            let name = get_component_name(info.name());
            ui.collapsing(name, |ui| {
                if let Some(f) = component_uis.0.get(info.type_id().as_ref().unwrap()) {
                    f(ui, world.entity_mut(entity));
                } else {
                    ui.label("Editing not supported :(");
                }
            });
        }
    });
}

fn is_hidden_component(id: Option<TypeId>) -> bool {
    let Some(id) = id else {
        return false;
    };

    TypeId::of::<bevy::render::primitives::Aabb>() == id
        || TypeId::of::<ViewVisibility>() == id
        || TypeId::of::<InheritedVisibility>() == id
        || TypeId::of::<GlobalTransform>() == id
        || TypeId::of::<Children>() == id
        || TypeId::of::<Parent>() == id
        || TypeId::of::<bevy_mod_picking::focus::PickingInteraction>() == id
}

fn get_component_name(name: &str) -> String {
    let mut output = String::new();
    let mut queue = String::new();
    for c in name.chars() {
        if c == ':' {
            queue.clear();
            continue;
        } else if c == '<' || c == '>' {
            output.push_str(&queue);
            output.push(c);
            queue.clear();
            continue;
        }

        queue.push(c);
    }
    // TODO: a::b::c::hello<a::b::c::world> -> hello<world>

    output.push_str(&queue);

    output
}
