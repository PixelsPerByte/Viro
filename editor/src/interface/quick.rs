use bevy::prelude::*;
use bevy_egui::egui;

use crate::command::EditorCommands;

#[derive(Resource)]
pub struct QuickCommand {
    pub search: String,
}

pub fn show(world: &mut World, ctx: &mut egui::Context) {
    let Some(mut quick) = world.remove_resource::<QuickCommand>() else {
        return;
    };

    let close = world
        .resource_scope::<EditorCommands, _>(|world, commands| {
            egui::Window::new("Quick Commands")
                .collapsible(false)
                .show(ctx, |ui| show_inner(ui, world, &mut quick, &commands))
        })
        .map(|res| res.inner)
        .flatten()
        .unwrap_or(false);

    if !close {
        world.insert_resource(quick);
    }
}

fn show_inner(
    ui: &mut egui::Ui,
    world: &mut World,
    quick: &mut QuickCommand,
    commands: &EditorCommands,
) -> bool {
    ui.text_edit_singleline(&mut quick.search);

    let mut indices: Vec<(usize, usize)> = Vec::new();
    for (command, i) in commands.0.iter().zip(0..) {
        let v = strsim::levenshtein(&command.name, &quick.search);
        indices.push((v, i));
    }
    indices.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, i) in indices {
        let command = &commands.0[i];
        if !ui.button(&command.name).clicked() {
            continue;
        }

        if let Err(e) = world.run_system(command.system) {
            error!(
                "Failed to execute command {:?}\nReturned error: {:?}",
                command, e
            );
        }

        return true;
    }

    false
}
