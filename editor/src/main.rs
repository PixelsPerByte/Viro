mod camera;
mod command;
mod interface;
mod observers;
mod picking;
mod transform;
mod transform_gizmo;

use bevy::{color::palettes::css::GOLD, prelude::*, render::primitives::Aabb, utils::HashSet};
use camera::{Flycam, FlycamPlugin};
use command::CommandPlugin;
use interface::InterfacePlugin;
use observers::ObserverPlugin;
use picking::PickingPlugin;

#[derive(Component)]
pub struct EditorEntity;

#[derive(Resource)]
pub struct SelectedEntities(pub HashSet<Entity>);

fn main() {
    let mut app = App::new();

    app.insert_resource(SelectedEntities(HashSet::default()));

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
    app.add_systems(Startup, (setup, setup_example));
    app.add_systems(PreUpdate, keybindings);
    app.add_systems(PostUpdate, selection_outlines);
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

fn setup_example(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(0.25, 0.25, 1.0),
                    ..default()
                }),
                ..default()
            },
            Name::new("Cuboid"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.25)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::linear_rgb(1.0, 1.0, 1.0),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                },
                Name::new("Sphere"),
            ));
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE * 3.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgb(0.25, 1.0, 0.25),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        Name::new("Plane"),
    ));

    commands.spawn((
        SpotLightBundle {
            transform: Transform::from_xyz(-5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("SpotLight"),
    ));
}

fn keybindings(
    keys: Res<ButtonInput<KeyCode>>,
    quick_command: Option<Res<interface::quick::QuickCommand>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) && quick_command.is_none() {
        commands.insert_resource(interface::quick::QuickCommand {
            search: String::new(),
        });
    }

    if keys.just_pressed(KeyCode::KeyG) {
        commands.trigger(transform::TransformSelected::Translate);
    } else if keys.just_pressed(KeyCode::KeyR) {
        commands.trigger(transform::TransformSelected::Rotate);
    } else if keys.just_pressed(KeyCode::KeyS) {
        commands.trigger(transform::TransformSelected::Scale);
    }
}

fn selection_outlines(
    selected: Res<SelectedEntities>,
    query: Query<(&Transform, &Aabb)>,
    mut gizmos: Gizmos,
) {
    // TODO: Should this be replaced with an outline shader?
    for entity in selected.0.iter() {
        let Ok((transform, aabb)) = query.get(*entity) else {
            continue;
        };

        gizmos
            .rounded_cuboid(
                transform.transform_point(aabb.center.into()),
                transform.rotation,
                Into::<Vec3>::into(aabb.half_extents * 2.0) * transform.scale,
                GOLD,
            )
            .edge_radius(0.0)
            .arc_resolution(0);
    }
}

fn grid(mut gizmos: Gizmos) {
    gizmos.line(
        Vec3::X * 10.0,
        -Vec3::X * 10.0,
        LinearRgba::rgb(1.0, 0.0, 0.0),
    );
    gizmos.line(
        Vec3::Y * 10.0,
        -Vec3::Y * 10.0,
        LinearRgba::rgb(0.0, 1.0, 0.0),
    );
    gizmos.line(
        Vec3::Z * 10.0,
        -Vec3::Z * 10.0,
        LinearRgba::rgb(0.0, 0.0, 1.0),
    );

    let mut grid_axis = |start: Vec3, end: Vec3, axis: Vec3| {
        for i in -10..=10 {
            if i == 0 {
                continue;
            }

            gizmos.line(
                start + i as f32 * axis,
                end + i as f32 * axis,
                LinearRgba::rgb(0.1, 0.1, 0.1),
            );
        }
    };

    grid_axis(Vec3::X * 10.0, Vec3::X * -10.0, Vec3::Z);
    grid_axis(Vec3::Z * 10.0, Vec3::Z * -10.0, Vec3::X);
}
