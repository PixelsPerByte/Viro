use bevy::prelude::*;

use crate::observers::SelectEntity;

pub fn mesh<M: Into<Mesh> + TypePath + Default>(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let mut entity_cmds = commands.spawn(PbrBundle {
        mesh: meshes.add(M::default()),
        material: materials.add(StandardMaterial::default()),
        ..default()
    });

    if let Some(ident) = M::type_ident() {
        entity_cmds.insert(Name::new(ident));
    }

    let target = entity_cmds.id();
    commands.trigger(SelectEntity { target });
}

pub fn point_light(mut commands: Commands) {
    let target = commands
        .spawn((PointLightBundle::default(), Name::new("PointLight")))
        .id();
    commands.trigger(SelectEntity { target });
}

pub fn spot_light(mut commands: Commands) {
    let target = commands
        .spawn((SpotLightBundle::default(), Name::new("SpotLight")))
        .id();
    commands.trigger(SelectEntity { target });
}

pub fn directional_light(mut commands: Commands) {
    let target = commands
        .spawn((
            DirectionalLightBundle::default(),
            Name::new("DirectionalLight"),
        ))
        .id();
    commands.trigger(SelectEntity { target });
}
