use bevy::{input::mouse::MouseMotion, prelude::*, utils::HashMap};

#[derive(Event, Clone)]
pub enum TransformSelected {
    Translate,
    Rotate,
    Scale,
}

#[derive(Resource)]
pub struct TransformEntities {
    pub entities: HashMap<Entity, Vec3>,
    pub delta: Vec2,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub mode: TransformSelected,
    pub center: Vec3,
}

pub fn update_delta(
    mut mouse_motion: EventReader<MouseMotion>,
    mut transform_entities: ResMut<TransformEntities>,
) {
    for motion in mouse_motion.read() {
        transform_entities.delta.x += motion.delta.x * 0.01;
        transform_entities.delta.y -= motion.delta.y * 0.01;
    }
}

pub fn update_transform(
    transform_entities: Res<TransformEntities>,
    mut transform_query: Query<&mut Transform>,
) {
    let offset = transform_entities.x_axis * transform_entities.delta.x
        + transform_entities.y_axis * transform_entities.delta.y;

    for (entity, home) in transform_entities.entities.iter() {
        let Ok(mut transform) = transform_query.get_mut(*entity) else {
            continue;
        };

        match transform_entities.mode {
            TransformSelected::Translate => {
                transform.translation = *home + offset;
            }
            TransformSelected::Rotate => {
                let euler = *home + offset;
                transform.rotation = Quat::from_euler(EulerRot::XYZ, euler.x, euler.y, euler.z);
            }
            TransformSelected::Scale => {
                transform.scale = *home + offset;
            }
        }
    }
}

pub fn finish_transform(
    _transform_entities: Res<TransformEntities>,
    _transform_query: Query<&Transform>,
    mut commands: Commands,
) {
    // TODO: Record History
    commands.remove_resource::<TransformEntities>();
}

pub fn cancel_transform(
    transform_entities: Res<TransformEntities>,
    mut transform_query: Query<&mut Transform>,
    mut commands: Commands,
) {
    for (&entity, &home) in transform_entities.entities.iter() {
        let Ok(mut transform) = transform_query.get_mut(entity) else {
            continue;
        };

        match transform_entities.mode {
            TransformSelected::Translate => {
                transform.translation = home;
            }
            TransformSelected::Rotate => {
                transform.rotation = Quat::from_euler(EulerRot::XYZ, home.x, home.y, home.z);
            }
            TransformSelected::Scale => {
                transform.scale = home;
            }
        }
    }

    commands.remove_resource::<TransformEntities>();
}

pub struct TransformPlugin;
impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            update_delta.run_if(resource_exists::<TransformEntities>),
        );
        app.add_systems(
            Update,
            update_transform.run_if(resource_exists::<TransformEntities>),
        );
    }
}
