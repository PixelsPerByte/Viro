use bevy::prelude::*;

use crate::{EditorAction, EditorEntity, SelectedEntities};

pub fn setup(mut commands: Commands) {
    commands.spawn((Observer::new(select_entity), EditorEntity));
}

#[derive(Event)]
pub struct SelectEntity {
    pub target: Entity,
}

pub fn select_entity(
    trigger: Trigger<SelectEntity>,
    editor_action: Res<EditorAction>,
    mut selected: ResMut<SelectedEntities>,
) {
    if editor_action.is_some_and(|v| v != crate::GUI_ACTION_ID) {
        return;
    }

    let event = trigger.event();

    if !selected.0.remove(&event.target) {
        selected.0.insert(event.target);
    }
}

pub struct ObserverPlugin;
impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}
