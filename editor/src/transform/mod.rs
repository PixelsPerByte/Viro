mod gizmo;
mod input;
mod observers;

use bevy::{prelude::*, utils::HashMap};

#[derive(Event, Clone)]
pub enum TransformMode {
    Translate {
        delta: Vec2,
        x_axis: Vec3,
        y_axis: Vec3,
    },
    Rotate {
        delta: f32,
        axis: Vec3,
    },
    Scale {
        delta: f32,
        axis: Vec3,
    },
}

pub enum TransformHome {
    Vec3(Vec3),
    Quat(Quat),
}

impl TransformHome {
    pub fn as_vec3(&self) -> Vec3 {
        match self {
            TransformHome::Vec3(v) => *v,
            TransformHome::Quat(_) => panic!("called `as_vec3` on Quat"),
        }
    }

    pub fn as_quat(&self) -> Quat {
        match self {
            TransformHome::Vec3(_) => panic!("called `as_quat` on Vec3"),
            TransformHome::Quat(q) => *q,
        }
    }
}

#[derive(Resource)]
pub struct TransformEntities {
    pub entities: HashMap<Entity, TransformHome>,
    pub mode: TransformMode,
    pub center: Vec3,
}

pub fn update_transform(
    transform_entities: Res<TransformEntities>,
    keys: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<&mut Transform>,
) {
    let is_snapping = keys.pressed(KeyCode::ControlLeft);

    for (entity, home) in transform_entities.entities.iter() {
        let Ok(mut transform) = transform_query.get_mut(*entity) else {
            continue;
        };

        match transform_entities.mode {
            TransformMode::Translate {
                delta,
                x_axis,
                y_axis,
            } => {
                let snapped_delta = if is_snapping { delta.trunc() } else { delta };
                let offset = x_axis * snapped_delta.x + y_axis * snapped_delta.y;

                transform.translation = home.as_vec3() + offset;
            }
            TransformMode::Rotate { delta, axis } => {
                let snapped_delta = if is_snapping { delta.trunc() } else { delta };
                let offset = Quat::from_axis_angle(axis, snapped_delta);

                transform.rotation = home.as_quat() * offset;
            }
            TransformMode::Scale { delta, axis } => {
                let snapped_delta = if is_snapping { delta.trunc() } else { delta };
                let offset = axis * snapped_delta;

                transform.scale = home.as_vec3() + offset;
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
    for (&entity, home) in transform_entities.entities.iter() {
        let Ok(mut transform) = transform_query.get_mut(entity) else {
            continue;
        };

        match transform_entities.mode {
            TransformMode::Translate { .. } => {
                transform.translation = home.as_vec3();
            }
            TransformMode::Rotate { .. } => {
                transform.rotation = home.as_quat();
            }
            TransformMode::Scale { .. } => {
                transform.scale = home.as_vec3();
            }
        }
    }

    commands.remove_resource::<TransformEntities>();
}

pub struct TransformPlugin;
impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, observers::setup);
        app.add_systems(
            PreUpdate,
            (
                input::update,
                input::update_delta.run_if(resource_exists::<TransformEntities>),
            ),
        );
        app.add_systems(
            Update,
            update_transform.run_if(resource_exists::<TransformEntities>),
        );
    }
}
