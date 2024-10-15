use std::{fs::File, io::Write, path::PathBuf};

use bevy::{prelude::*, tasks::IoTaskPool};

use crate::EditorEntity;

#[derive(Resource, Deref, DerefMut)]
pub struct SceneFilePath(pub PathBuf);

pub fn open(world: &mut World) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("bevy scene", &["scn.ron"])
        .pick_file()
    else {
        error!("Failed to get file path.");
        return;
    };

    let scene = world.load_asset(path);

    world.spawn(SceneBundle { scene, ..default() });
}

pub fn save<const AS: bool>(world: &mut World) {
    if AS || !world.contains_resource::<SceneFilePath>() {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("bevy scene", &["scn.ron"])
            .save_file()
        else {
            error!("Failed to get file path.");
            return;
        };

        world.insert_resource(SceneFilePath(path));
    }

    let mut scene_builder = DynamicSceneBuilder::from_world(world)
        .allow_all()
        // .deny::<Handle<Mesh>>()
        // .deny::<Handle<StandardMaterial>>()
        .deny_all_resources()
        .allow_resource::<Assets<Mesh>>()
        .allow_resource::<Assets<StandardMaterial>>()
        .extract_resources();

    for entity in world.iter_entities() {
        if entity.contains::<EditorEntity>()
            || entity.contains::<Window>()
            || entity.contains::<bevy_mod_picking::prelude::PointerId>()
        {
            continue;
        }

        scene_builder = scene_builder.extract_entity(entity.id());
    }

    let scene = scene_builder.build();
    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap(); // FIXME: Dont Panic

    let path = world.resource::<SceneFilePath>().0.clone();

    IoTaskPool::get()
        .spawn(async move {
            if let Err(e) =
                File::create(&path).and_then(|mut file| file.write(serialized_scene.as_bytes()))
            {
                error!("Failed to write scene to file: {path:?}\n{e:?}");
            }
        })
        .detach();
}
