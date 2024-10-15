use bevy::prelude::*;

pub fn system<M: Into<Mesh> + Default>(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(M::default()),
        material: materials.add(StandardMaterial::default()),
        ..default()
    });
}
