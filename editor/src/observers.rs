use bevy::prelude::*;

use crate::{EditorAction, EditorEntity, SelectedEntities};

pub fn setup(mut commands: Commands) {
    commands.spawn((Observer::new(select_entity), EditorEntity));
    commands.spawn((Observer::new(delete_selected), EditorEntity));
}

#[derive(Event)]
pub struct SelectEntity {
    pub target: Entity,
}

pub fn select_entity(
    trigger: Trigger<SelectEntity>,
    editor_action: Res<EditorAction>,
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedEntities>,
) {
    if editor_action.is_some_and(|v| v != crate::GUI_ACTION_ID) {
        return;
    }

    let event = trigger.event();
    if selected.0.swap_remove(&event.target) {
        return;
    }

    if !keys.pressed(KeyCode::ShiftLeft) {
        selected.0.clear();
    }
    selected.0.insert(event.target);
}

#[derive(Event)]
pub struct DeleteSelected;

fn delete_selected(
    _trigger: Trigger<DeleteSelected>,
    mut selected: ResMut<SelectedEntities>,
    mut commands: Commands,
) {
    while let Some(entity) = selected.0.pop() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct ObserverPlugin;
impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}
