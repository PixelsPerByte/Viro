use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{EditorAction, TRANSFORM_ACTION_ID};

use super::{observers, TransformEntities, TransformMode};

pub fn update(
    editor_action: Res<EditorAction>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut transform_entities: Option<ResMut<TransformEntities>>,
    mut commands: Commands,
) {
    if editor_action.is_none_or(|v| v == TRANSFORM_ACTION_ID) {
        if keys.just_pressed(KeyCode::KeyG) {
            commands.trigger(observers::TransformSelected::Translate);
        } else if keys.just_pressed(KeyCode::KeyR) {
            commands.trigger(observers::TransformSelected::Rotate);
        } else if keys.just_pressed(KeyCode::KeyS) {
            commands.trigger(observers::TransformSelected::Scale);
        }
    }

    if !editor_action.is_some_and(|v| v == TRANSFORM_ACTION_ID) {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        commands.trigger(observers::FinishTransform);
    } else if mouse_button.just_pressed(MouseButton::Right) {
        commands.trigger(observers::CancelTransform);
    }

    if keys.just_pressed(KeyCode::Escape) {
        commands.trigger(observers::CancelTransform);
    }

    // Change Axis
    let mut new_axis: Option<Vec3> = if keys.just_pressed(KeyCode::KeyX) {
        Some(Vec3::X)
    } else if keys.just_pressed(KeyCode::KeyY) {
        Some(Vec3::Y)
    } else if keys.just_pressed(KeyCode::KeyZ) {
        Some(Vec3::Z)
    } else {
        None
    };

    if let Some(new_axis) = &mut new_axis {
        if keys.pressed(KeyCode::ShiftLeft) {
            *new_axis = Vec3::ONE - *new_axis;
        }

        let transform_entities = transform_entities.as_mut().unwrap();
        match &mut transform_entities.mode {
            TransformMode::Translate { x_axis, y_axis, .. } => {
                *x_axis = *new_axis;
                *y_axis = Vec3::ZERO;
            }
            TransformMode::Rotate { axis, .. } | TransformMode::Scale { axis, .. } => {
                *axis = *new_axis;
            }
        }
    }
}

pub fn update_delta(
    mut mouse_motion: EventReader<MouseMotion>,
    mut transform_entities: ResMut<TransformEntities>,
) {
    let mouse_delta = mouse_motion.read().fold(Vec2::ZERO, |a, b| a + b.delta);

    match &mut transform_entities.mode {
        TransformMode::Translate { delta, .. } => {
            delta.x += mouse_delta.x * 0.01;
            delta.y -= mouse_delta.y * 0.01;
        }
        TransformMode::Rotate { delta, .. } => {
            *delta += mouse_delta.x.atan() * 0.01 + mouse_delta.y.atan() * 0.01;
        }
        TransformMode::Scale { delta, .. } => {
            *delta += mouse_delta.x * 0.01;
        }
    }
}
