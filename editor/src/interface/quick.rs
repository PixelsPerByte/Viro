use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{command::EditorCommands, EditorAction};

use super::InterfaceSet;

#[derive(Resource, Default)]
pub struct QuickCommand {
    pub search: String,
    pub selected: usize,
}

fn show(
    commands: Res<EditorCommands>,
    mut quick: ResMut<QuickCommand>,
    mut editor_action: ResMut<EditorAction>,
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
        editor_action.0 = None;
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
    let response = ui.add(search_field);
    response.request_focus();
    if response.changed() {
        quick.selected = 0;
    }

    // Sort commands by levenshtein distance from search field
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for (command, i) in commands.list.iter().zip(0..) {
        let v = strsim::levenshtein(&command.name, &quick.search);
        indices.push((v, i));
    }
    indices.sort_by(|a, b| a.0.cmp(&b.0));

    // Keybindings
    let enter_pressed = ui.input(|state| {
        if state.key_pressed(egui::Key::ArrowDown) {
            quick.selected = (quick.selected + 1).min(indices.len() - 1);
        } else if state.key_pressed(egui::Key::ArrowUp) {
            quick.selected = quick.selected.saturating_sub(1);
        }

        state.key_pressed(egui::Key::Enter)
    });

    if enter_pressed {
        let index = indices[quick.selected].1;
        world_commands.run_system(commands.list[index].system);
        return true;
    }

    // Show commands
    for (bi, (_, i)) in indices.iter().enumerate() {
        let command = &commands.list[*i];
        let mut button = egui::Button::new(&command.name)
            .fill(egui::Color32::TRANSPARENT)
            .min_size(egui::vec2(ui.available_width(), 5.0))
            .selected(bi == quick.selected);

        if let Some(path) = command.toolbar.as_ref() {
            button = button.shortcut_text(path);
        }

        if ui.add(button).clicked() {
            world_commands.run_system(command.system);
            return true;
        }
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
