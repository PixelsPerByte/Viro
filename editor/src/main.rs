mod camera;
mod command;
mod interface;
mod observers;
mod picking;
mod transform;

use bevy::{color::palettes::css::GOLD, prelude::*, render::primitives::Aabb, utils::HashSet};
use camera::{Flycam, FlycamPlugin};
use command::CommandPlugin;
use indexmap::IndexSet;
use interface::InterfacePlugin;
use observers::ObserverPlugin;
use picking::PickingPlugin;

pub const GUI_ACTION_ID: u64 = 0;
pub const CAMERA_ACTION_ID: u64 = 1;
pub const TRANSFORM_ACTION_ID: u64 = 2;
pub const QUICK_COMMANDS_ACTION_ID: u64 = 3;

#[derive(Resource, Deref)]
pub struct EditorAction(pub Option<u64>);

#[derive(Component)]
pub struct EditorEntity;

#[derive(Resource)]
pub struct SelectedEntities(pub IndexSet<Entity>);

fn main() {
    let mut app = App::new();

    app.insert_resource(SelectedEntities(IndexSet::default()));
    app.insert_resource(EditorAction(None));

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Viro Editor".into(),
                ..default()
            }),
            ..default()
        }),
        FlycamPlugin,
        ObserverPlugin,
        InterfacePlugin,
        PickingPlugin,
        transform::TransformPlugin,
        CommandPlugin,
    ));
    app.add_systems(Startup, setup);
    app.add_systems(PreUpdate, keybindings);
    app.add_systems(PostUpdate, (grid, selection_outlines));
    app.run();
}

fn setup(mut commands: Commands, mut gizmo_config_store: ResMut<GizmoConfigStore>) {
    let (_gizmo_config, gizmo_light_config) =
        gizmo_config_store.config_mut::<LightGizmoConfigGroup>();
    gizmo_light_config.draw_all = true;

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Flycam::default(),
        EditorEntity,
    ));
}

fn keybindings(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut editor_action: ResMut<EditorAction>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) && editor_action.is_none_or(|v| v == GUI_ACTION_ID) {
        commands.insert_resource(interface::quick::QuickCommand::default());
        editor_action.0 = Some(QUICK_COMMANDS_ACTION_ID);
    } else if keys.just_pressed(KeyCode::Escape)
        && editor_action.is_some_and(|v| v == QUICK_COMMANDS_ACTION_ID)
    {
        commands.remove_resource::<interface::quick::QuickCommand>();
        editor_action.0 = None;
    }

    if keys.just_pressed(KeyCode::Delete) && editor_action.is_none() {
        commands.trigger(observers::DeleteSelected);
    }
}

fn selection_outlines(
    selected: Res<SelectedEntities>,
    query: Query<&Aabb>,
    transform_helper: TransformHelper,
    mut gizmos: Gizmos,
) {
    // TODO: Should this be replaced with an outline shader?
    for &entity in selected.0.iter() {
        let Ok(global_transform) = transform_helper.compute_global_transform(entity) else {
            continue;
        };

        let Ok(aabb) = query.get(entity) else {
            continue;
        };

        let (scale, rotation, _translation) = global_transform.to_scale_rotation_translation();

        gizmos
            .rounded_cuboid(
                global_transform.transform_point(aabb.center.into()),
                rotation,
                Into::<Vec3>::into(aabb.half_extents * 2.0) * scale,
                GOLD,
            )
            .edge_radius(0.0)
            .arc_resolution(0);
    }
}

fn grid(camera_query: Query<&Transform, With<Flycam>>, mut gizmos: Gizmos) {
    let camera = camera_query.get_single().unwrap();

    gizmos.line(
        Vec3::X * 1000.0 + Vec3::Y * 0.001,
        -Vec3::X * 1000.0 + Vec3::Y * 0.001,
        LinearRgba::rgb(1.0, 0.0, 0.0),
    );
    gizmos.line(
        Vec3::Y * 1000.0,
        -Vec3::Y * 1000.0,
        LinearRgba::rgb(0.0, 1.0, 0.0),
    );
    gizmos.line(
        Vec3::Z * 1000.0 + Vec3::Y * 0.001,
        -Vec3::Z * 1000.0 + Vec3::Y * 0.001,
        LinearRgba::rgb(0.0, 0.0, 1.0),
    );

    let mut grid_axis = |center: Vec3, length: f32, axis: Vec3, offset: Vec3| {
        for i in -200..=200 {
            let mut start_color = LinearRgba::rgb(0.1, 0.1, 0.1);
            let end_color = LinearRgba::new(0.1, 0.1, 0.1, 0.0);
            start_color.alpha = 1.0 - (i as f32).abs() * 0.005;

            let offset = i as f32 * offset;
            let start = axis * length;
            let end = axis * -length;

            gizmos.line_gradient(
                offset + center,
                start + offset + center,
                start_color,
                end_color,
            );
            gizmos.line_gradient(
                offset + center,
                end + offset + center,
                start_color,
                end_color,
            );
        }
    };

    let translation = camera.translation.trunc() * Vec3::new(1.0, 0.0, 1.0);

    grid_axis(translation, 210.0, Vec3::X, Vec3::Z);
    grid_axis(translation, 210.0, Vec3::Z, Vec3::X);
}
