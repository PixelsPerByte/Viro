use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::observers::SelectEntity;

pub fn pointer_select(mut event_reader: EventReader<Pointer<Click>>, mut commands: Commands) {
    for event in event_reader.read() {
        if event.button == PointerButton::Primary {
            commands.trigger(SelectEntity {
                target: event.target,
            });
        }
    }
}

pub struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);
        app.add_systems(PreUpdate, pointer_select);
    }
}
