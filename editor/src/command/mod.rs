mod add;
mod import;
mod scene;

use bevy::{ecs::system::SystemId, prelude::*};
use indexmap::IndexMap;

use crate::EditorEntity;

fn register_command<M, S: IntoSystem<(), (), M> + 'static>(
    world: &mut World,
    name: String,
    toolbar: Option<&str>,
    system: S,
) -> usize {
    let system = world.register_system(system);
    world.entity_mut(system.entity()).insert(EditorEntity);

    let mut commands = world.resource_mut::<EditorCommands>();
    let action_index = commands.list.len();
    commands.list.push(EditorCommand {
        name: name.clone(),
        toolbar: toolbar.map(|s| s.to_string()),
        system,
    });

    // Register in toolbar
    if let Some(path) = toolbar {
        register_toolbar(world, name, path, action_index);
    }

    action_index
}

fn register_toolbar(world: &mut World, name: String, path: &str, action_index: usize) {
    let mut commands = world.resource_mut::<EditorCommands>();

    let mut section = match &mut commands.toolbar {
        ToolBar::Section(section) => section,
        ToolBar::Action(_) => unreachable!("Root toolbar is an action"),
    };

    for id in path.split('/') {
        let toolbar = section
            .entry(id.to_string())
            .or_insert(ToolBar::Section(IndexMap::new()));

        match toolbar {
            ToolBar::Section(new_section) => {
                section = new_section;
            }
            ToolBar::Action(_) => {
                error!("Encountered an action while searching for toolbar section `{path}`");
                return;
            }
        }
    }

    section.insert(name, ToolBar::Action(action_index));
}

#[derive(Debug)]
pub struct EditorCommand {
    /// The name of the command.
    /// This shows in the "Quick Commands" and toolbar.
    pub name: String,

    /// The path of this command in the toolbar (shown in quick commands)
    pub toolbar: Option<String>,

    /// The system to execute
    pub system: SystemId,
}

pub enum ToolBar {
    Section(IndexMap<String, ToolBar>),
    Action(usize),
}

#[derive(Resource)]
pub struct EditorCommands {
    pub list: Vec<EditorCommand>,
    pub toolbar: ToolBar,
}

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorCommands {
            list: Vec::new(),
            toolbar: ToolBar::Section(IndexMap::new()),
        });

        register_command(app.world_mut(), "Open".into(), Some("File"), scene::open);
        register_command(
            app.world_mut(),
            "Save".into(),
            Some("File"),
            scene::save::<false>,
        );
        register_command(
            app.world_mut(),
            "Save As".into(),
            Some("File"),
            scene::save::<true>,
        );

        register_command(
            app.world_mut(),
            "Gltf".into(),
            Some("File/Import"),
            import::gltf,
        );
        // register_command(
        //     app.world_mut(),
        //     "Import VRM".into(),
        //     ToolbarSection::Import,
        //     import::vrm,
        // );

        // Primitives
        register_command(
            app.world_mut(),
            "Cube".into(),
            Some("Add"),
            add::mesh::<Cuboid>,
        );
        register_command(
            app.world_mut(),
            "Sphere".into(),
            Some("Add"),
            add::mesh::<Sphere>,
        );
        register_command(
            app.world_mut(),
            "Plane".into(),
            Some("Add"),
            add::mesh::<Plane3d>,
        );
        register_command(
            app.world_mut(),
            "Cylinder".into(),
            Some("Add"),
            add::mesh::<Cylinder>,
        );
        register_command(
            app.world_mut(),
            "Cone".into(),
            Some("Add"),
            add::mesh::<Cone>,
        );
        register_command(
            app.world_mut(),
            "Torus".into(),
            Some("Add"),
            add::mesh::<Torus>,
        );

        register_command(
            app.world_mut(),
            "Directional Light".into(),
            Some("Add"),
            add::directional_light,
        );
        register_command(
            app.world_mut(),
            "Point Light".into(),
            Some("Add"),
            add::point_light,
        );
        register_command(
            app.world_mut(),
            "Spot Light".into(),
            Some("Add"),
            add::spot_light,
        );
    }
}
