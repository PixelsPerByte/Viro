use bevy::{prelude::*, utils::HashMap};

use crate::{camera::Flycam, EditorAction, EditorEntity, SelectedEntities};

use super::{TransformEntities, TransformHome, TransformMode};

#[derive(Event)]
pub enum TransformSelected {
    Translate,
    Rotate,
    Scale,
}

#[derive(Event)]
pub struct FinishTransform;

#[derive(Event)]
pub struct CancelTransform;

pub fn setup(mut commands: Commands) {
    commands.spawn((Observer::new(transform_selected), EditorEntity));
    commands.spawn((Observer::new(transform_finish), EditorEntity));
    commands.spawn((Observer::new(transform_cancel), EditorEntity));
}

pub fn transform_selected(
    trigger: Trigger<TransformSelected>,
    selected: Res<SelectedEntities>,
    transform_query: Query<&Transform>,
    camera_query: Query<&Transform, With<Flycam>>,
    transform_entities: Option<Res<TransformEntities>>,
    mut editor_action: ResMut<EditorAction>,
    mut commands: Commands,
) {
    if transform_entities.is_some() {
        return;
    }

    if selected.0.is_empty() {
        info!("No entity is selected.");
        return;
    }

    if !editor_action.is_none_or(|v| v == crate::TRANSFORM_ACTION_ID) {
        return;
    }

    let camera_rotation = camera_query
        .get_single()
        .expect("Editor Camera doesn't exist.")
        .rotation;

    // Get mode and axises
    let mode = match trigger.event() {
        TransformSelected::Translate => TransformMode::Translate {
            delta: Vec2::ZERO,
            x_axis: camera_rotation * Vec3::X,
            y_axis: camera_rotation * Vec3::Y,
        },
        TransformSelected::Rotate => TransformMode::Rotate {
            delta: 0.0,
            axis: camera_rotation * Vec3::Z,
        },
        TransformSelected::Scale => TransformMode::Scale {
            delta: 0.0,
            axis: Vec3::ONE,
        },
    };

    let mut resource = TransformEntities {
        entities: HashMap::new(),
        mode,
        center: Vec3::ZERO,
    };

    for entity in selected.0.iter() {
        let Ok(transform) = transform_query.get(*entity) else {
            info!("The transform operation has been cancelled.");
            warn!("A selected entity doesn't have a `Transform` component.");
            return;
        };

        let data = match trigger.event() {
            TransformSelected::Translate => TransformHome::Vec3(transform.translation),
            TransformSelected::Rotate => TransformHome::Quat(transform.rotation),
            TransformSelected::Scale => TransformHome::Vec3(transform.scale),
        };

        resource.entities.insert(*entity, data);
        resource.center += transform.translation;
    }

    resource.center /= selected.0.len() as f32;
    commands.insert_resource(resource);
    editor_action.0 = Some(crate::TRANSFORM_ACTION_ID);
}

pub fn transform_finish(
    _trigger: Trigger<FinishTransform>,
    transform_entities: Res<TransformEntities>,
    transform_query: Query<&Transform>,
    commands: Commands,
    mut editor_action: ResMut<EditorAction>,
) {
    super::finish_transform(transform_entities, transform_query, commands);
    editor_action.0 = None;
}

pub fn transform_cancel(
    _trigger: Trigger<CancelTransform>,
    transform_entities: Res<TransformEntities>,
    transform_query: Query<&mut Transform>,
    commands: Commands,
    mut editor_action: ResMut<EditorAction>,
) {
    super::cancel_transform(transform_entities, transform_query, commands);
    editor_action.0 = None;
}
