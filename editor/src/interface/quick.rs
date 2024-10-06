use bevy::prelude::*;
use bevy_egui::egui;

use crate::command::EditorCommands;

pub fn show(world: &mut World, ui: &mut egui::Ui) {
    world.resource_scope::<EditorCommands, _>(|world, commands| {
        for command in commands.0.iter() {
            if !ui.button(&command.name).clicked() {
                continue;
            }

            if let Err(e) = world.run_system(command.system) {
                error!(
                    "Failed to execute command {:?}\nReturned error: {:?}",
                    command, e
                );
            }
        }
    });
}
