use bevy::prelude::*;

pub fn system<M: Into<Mesh> + Default>(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(M::default()),
        ..default()
    });
}
