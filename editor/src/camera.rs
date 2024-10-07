use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::EditorFocus;

#[derive(Component)]
pub struct Flycam {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            speed: 4.0,
            sensitivity: 0.01,
        }
    }
}

fn update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&Flycam, &mut Transform)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut editor_focus: ResMut<EditorFocus>,
) {
    let Ok(mut window) = window_query.get_single_mut() else {
        return;
    };

    // the first check is a fancy "is right click pressed",
    // but only if right click was pressed while the cursor was not over any Ui
    if (!mouse_button.just_pressed(MouseButton::Right) && window.cursor.visible)
        || mouse_button.just_released(MouseButton::Right)
        || !matches!(
            editor_focus.as_ref(),
            &EditorFocus::Camera | &EditorFocus::None
        )
    {
        if !window.cursor.visible {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }

        if matches!(editor_focus.as_ref(), &EditorFocus::Camera) {
            *editor_focus = EditorFocus::None;
        }
        return;
    }

    // Lock Cursor
    if window.cursor.visible {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    // Get Input
    let mouse_delta = mouse_motion
        .read()
        .fold(Vec2::ZERO, |o, m| o - m.delta.yx());

    let forward = keys.pressed(KeyCode::KeyW) as i8 - keys.pressed(KeyCode::KeyS) as i8;
    let right = keys.pressed(KeyCode::KeyD) as i8 - keys.pressed(KeyCode::KeyA) as i8;
    let up = keys.pressed(KeyCode::KeyE) as i8 - keys.pressed(KeyCode::KeyQ) as i8;
    let movement = Vec3::new(right as f32, up as f32, forward as f32) * time.delta_seconds();

    // Apply transform
    for (flycam, mut transform) in query.iter_mut() {
        // Rotation
        let (ry, rx, _rz) = transform.rotation.to_euler(EulerRot::YXZ);
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            ry + mouse_delta.y * flycam.sensitivity,
            rx + mouse_delta.x * flycam.sensitivity,
            0.0,
        );

        // Translation
        let translation_delta = (transform.right() * movement.x
            + transform.up() * movement.y
            + transform.forward() * movement.z)
            * flycam.speed;
        transform.translation += translation_delta;
    }

    *editor_focus = EditorFocus::Camera;
}

pub struct FlycamPlugin;
impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}
