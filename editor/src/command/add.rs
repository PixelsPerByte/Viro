use bevy::prelude::*;

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
}

pub fn point_light(mut commands: Commands) {
    commands.spawn((PointLightBundle::default(), Name::new("PointLight")));
}

pub fn spot_light(mut commands: Commands) {
    commands.spawn((SpotLightBundle::default(), Name::new("SpotLight")));
}

pub fn directional_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle::default(),
        Name::new("DirectionalLight"),
    ));
}
