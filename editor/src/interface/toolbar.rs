use bevy::{prelude::*, utils::HashMap};
use bevy_egui::egui;

use crate::command::{EditorCommands, ToolbarSection};

#[derive(Deref, DerefMut)]
struct Buttons(HashMap<String, (Vec<usize>, Box<Buttons>)>);

pub fn show(world: &mut World, ui: &mut egui::Ui) {
    world.resource_scope::<EditorCommands, _>(|world, commands| {
        let mut sections: HashMap<ToolbarSection, Vec<usize>> = HashMap::new();
        for (command, i) in commands.0.iter().zip(0..) {
            if matches!(command.toolbar, ToolbarSection::None) {
                continue;
            }

            let section = sections.entry(command.toolbar).or_insert(Vec::new());
            section.push(i);
        }

        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                let Some(section) = sections.get(&ToolbarSection::File) else {
                    return;
                };

                show_section(world, ui, &commands, section);
            });

            ui.menu_button("Import", |ui| {
                let Some(section) = sections.get(&ToolbarSection::Import) else {
                    return;
                };

                show_section(world, ui, &commands, section);
            });
        });
    });
}

fn show_section(
    world: &mut World,
    ui: &mut egui::Ui,
    commands: &EditorCommands,
    section: &[usize],
) {
    for i in section {
        let command = &commands.0[*i];
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
