use bevy::{prelude::*, utils::HashMap};
use bevy_egui::egui;
use indexmap::IndexMap;

use crate::command::{EditorCommands, ToolBar};

#[derive(Deref, DerefMut)]
struct Buttons(HashMap<String, (Vec<usize>, Box<Buttons>)>);

pub fn show(world: &mut World, ui: &mut egui::Ui) {
    world.resource_scope::<EditorCommands, _>(|world, commands| {
        ui.horizontal(|ui| {
            let ToolBar::Section(section) = &commands.toolbar else {
                return;
            };

            show_section(world, ui, &commands, section);
        });
    });
}

fn show_section(
    world: &mut World,
    ui: &mut egui::Ui,
    commands: &EditorCommands,
    section: &IndexMap<String, ToolBar>,
) {
    for (name, toolbar) in section.iter() {
        match toolbar {
            ToolBar::Section(section) => {
                ui.menu_button(name, |ui| show_section(world, ui, commands, section));
            }
            ToolBar::Action(action) => {
                let command = &commands.list[*action];
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
        }
    }
}
