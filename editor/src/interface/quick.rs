use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::command::EditorCommands;

use super::InterfaceSet;

#[derive(Resource)]
pub struct QuickCommand {
    pub search: String,
}

fn show(
    commands: Res<EditorCommands>,
    mut quick: ResMut<QuickCommand>,
    mut contexts: EguiContexts,
    mut world_commands: Commands,
) {
    let ctx = contexts.ctx_mut();

    let close = egui::Window::new("Quick Commands")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .movable(false)
        .show(ctx, |ui| {
            show_inner(ui, &mut world_commands, &mut quick, &commands)
        })
        .map(|res| res.inner)
        .flatten()
        .unwrap_or(false);

    if close {
        world_commands.remove_resource::<QuickCommand>();
    }
}

fn show_inner(
    ui: &mut egui::Ui,
    world_commands: &mut Commands,
    quick: &mut QuickCommand,
    commands: &EditorCommands,
) -> bool {
    let search_field = egui::TextEdit::singleline(&mut quick.search)
        .min_size(egui::vec2(ui.available_width(), 5.0));
    ui.add(search_field);

    let mut indices: Vec<(usize, usize)> = Vec::new();
    for (command, i) in commands.list.iter().zip(0..) {
        let v = strsim::levenshtein(&command.name, &quick.search);
        indices.push((v, i));
    }
    indices.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, i) in indices {
        let command = &commands.list[i];
        let mut button = egui::Button::new(&command.name)
            .fill(egui::Color32::TRANSPARENT)
            .min_size(egui::vec2(ui.available_width(), 5.0));

        if let Some(path) = command.toolbar.as_ref() {
            button = button.shortcut_text(path);
        }

        if !ui.add(button).clicked() {
            continue;
        }

        world_commands.run_system(command.system);
        // if let Err(e) =  {
        //     error!(
        //         "Failed to execute command {:?}\nReturned error: {:?}",
        //         command, e
        //     );
        // }

        return true;
    }

    false
}

pub struct QuickCommandPlugin;
impl Plugin for QuickCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            show.run_if(resource_exists::<QuickCommand>)
                .in_set(InterfaceSet::Overlay),
        );
    }
}
