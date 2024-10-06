mod import;
mod scene;

use bevy::{ecs::system::SystemId, prelude::*};

use crate::EditorEntity;

fn register_command<M, S: IntoSystem<(), (), M> + 'static>(
    world: &mut World,
    name: String,
    toolbar: ToolbarSection,
    system: S,
) {
    let system = world.register_system(system);
    world.entity_mut(system.entity()).insert(EditorEntity);

    let mut commands = world.resource_mut::<EditorCommands>();
    commands.0.push(EditorCommand {
        name,
        toolbar,
        system,
    });
}

#[derive(Debug)]
pub struct EditorCommand {
    /// The name of the command.
    /// This shows in the "Quick Commands" and toolbar.
    pub name: String,

    pub toolbar: ToolbarSection,

    /// The system to execute
    pub system: SystemId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ToolbarSection {
    /// Only show in quick commands
    None,
    File,
    Import,
}

#[derive(Resource, Default)]
pub struct EditorCommands(pub Vec<EditorCommand>);

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorCommands::default());

        register_command(
            app.world_mut(),
            "Open".into(),
            ToolbarSection::File,
            scene::open,
        );
        register_command(
            app.world_mut(),
            "Save".into(),
            ToolbarSection::File,
            scene::save::<false>,
        );
        register_command(
            app.world_mut(),
            "Save As".into(),
            ToolbarSection::File,
            scene::save::<true>,
        );

        register_command(
            app.world_mut(),
            "Import Gltf".into(),
            ToolbarSection::Import,
            import::gltf,
        );
        // register_command(
        //     app.world_mut(),
        //     "Import VRM".into(),
        //     ToolbarSection::Import,
        //     import::vrm,
        // );
    }
}
