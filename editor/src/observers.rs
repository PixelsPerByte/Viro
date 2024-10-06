use bevy::prelude::*;

use crate::{EditorEntity, SelectedEntities};

pub fn setup(mut commands: Commands) {
    commands.spawn((Observer::new(select_entity), EditorEntity));
}

#[derive(Event)]
pub struct SelectEntity {
    pub target: Entity,
}

pub fn select_entity(trigger: Trigger<SelectEntity>, mut selected: ResMut<SelectedEntities>) {
    let event = trigger.event();

    if !selected.0.remove(&event.target) {
        selected.0.insert(event.target);
    }
}
