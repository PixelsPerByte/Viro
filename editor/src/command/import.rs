use bevy::prelude::*;

pub fn gltf(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut dialog = rfd::FileDialog::new().add_filter("Gltf", &["gltf", "glb"]);
    if let Ok(path) = std::env::current_dir() {
        dialog = dialog.set_directory(path);
    }

    let Some(path) = dialog.pick_file() else {
        return;
    };

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(path)),
        ..default()
    });
}

// pub fn vrm(asset_server: Res<AssetServer>, mut commands: Commands) {
//     let Some(path) = rfd::FileDialog::new()
//         .add_filter("Vrm", &["vrm"])
//         .pick_file()
//     else {
//         return;
//     };

//     commands.spawn(bevy_vrm::VrmBundle {
//         scene_bundle: SceneBundle { ..default() },
//         vrm: asset_server.load(path),
//         ..default()
//     });
// }
