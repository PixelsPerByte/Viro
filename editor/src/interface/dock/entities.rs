use bevy::{ecs::observer::ObserverState, prelude::*};
use bevy_egui::egui::{self, collapsing_header::CollapsingState};

use crate::{observers::SelectEntity, EditorEntity, SelectedEntities};

pub fn show(world: &mut World, ui: &mut egui::Ui) {
    let mut roots = world.query_filtered::<Entity, (Without<Parent>, Without<EditorEntity>)>();
    let mut info_query =
        world.query_filtered::<(Option<&Name>, Option<&Children>), Without<EditorEntity>>();

    let entities = roots.iter(world).collect::<Vec<Entity>>();
    for entity in entities {
        show_entity(ui, world, &mut info_query, entity);
    }
}

fn show_entity(
    ui: &mut egui::Ui,
    world: &mut World,
    info_query: &mut QueryState<(Option<&Name>, Option<&Children>), Without<EditorEntity>>,
    entity: Entity,
) {
    if is_hidden_entity(world.entity(entity)) {
        return;
    }

    let Ok((name, children)) = info_query.get(world, entity) else {
        return;
    };

    let is_selected = world.resource::<SelectedEntities>().0.contains(&entity);
    let label = if let Some(name) = name {
        format!("{} ({})", name, entity)
    } else {
        entity.to_string()
    };

    if let Some(children) = children {
        let children = children.iter().cloned().collect::<Vec<Entity>>();

        CollapsingState::load_with_default_open(ui.ctx(), entity.to_string().into(), false)
            .show_header(ui, |ui| {
                if ui.selectable_label(is_selected, label).clicked() {
                    world.trigger(SelectEntity { target: entity });
                }
            })
            .body(|ui| {
                for child in children {
                    show_entity(ui, world, info_query, child);
                }
            });
    } else {
        ui.horizontal(|ui| {
            ui.add_space(20.0); // Pad entities with no children to align
            if ui.selectable_label(is_selected, label).clicked() {
                world.trigger(SelectEntity { target: entity });
            }
        });
    }
}

fn is_hidden_entity(entity: EntityRef<'_>) -> bool {
    entity.contains::<Window>()
        || entity.contains::<bevy_mod_picking::pointer::PointerId>()
        || entity.contains::<ObserverState>()
}
