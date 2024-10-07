mod components;
mod dock;
pub mod quick;
mod toolbar;

use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSet};
use components::ComponentUis;
use dock::{InterfaceTab, InterfaceTabViewer};
use egui_dock::{DockArea, DockState, NodeIndex};

use crate::{camera::Flycam, EditorEntity, EditorFocus};

#[derive(Resource)]
pub struct InterfaceState {
    pub dock_state: DockState<InterfaceTab>,
    pub viewport_rect: egui::Rect,
    pub cursor_over_ui: bool,
}

impl InterfaceState {
    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        quick::show(world, ctx);

        egui::TopBottomPanel::top("ToolBar").show(ctx, |ui| {
            toolbar::show(world, ui);
        });

        DockArea::new(&mut self.dock_state).show(
            ctx,
            &mut InterfaceTabViewer {
                world,
                viewport_rect: &mut self.viewport_rect,
                cursor_over_ui: &mut self.cursor_over_ui,
            },
        );
    }
}

impl Default for InterfaceState {
    fn default() -> Self {
        let mut dock_state =
            DockState::new(vec![InterfaceTab::Viewport, InterfaceTab::ViewSettings]);
        let surface = dock_state.main_surface_mut();

        let [_viewport, entities] =
            surface.split_left(NodeIndex::root(), 0.2, vec![InterfaceTab::Entities]);

        let [_entities, _inspector] =
            surface.split_below(entities, 0.5, vec![InterfaceTab::Inspector]);

        Self {
            dock_state,
            viewport_rect: egui::Rect::NOTHING,
            cursor_over_ui: false,
        }
    }
}

// https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/crates/bevy-inspector-egui/examples/integrations/egui_dock.rs#L82C4-L82C18
fn show_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<InterfaceState, _>(|world, mut state| {
        state.ui(world, egui_context.get_mut());

        let mut editor_focus = world.get_resource_mut::<EditorFocus>().unwrap();
        if matches!(editor_focus.as_ref(), &EditorFocus::None) {
            *editor_focus = EditorFocus::Gui;
        }
    });
}

// https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/crates/bevy-inspector-egui/examples/integrations/egui_dock.rs#L97
fn set_camera_viewport(
    state: Res<InterfaceState>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Camera, (With<Flycam>, With<EditorEntity>)>,
) {
    let mut camera = camera_query.single_mut();
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let window_size = window.physical_size();
    let far_corner = physical_position + physical_size;

    if far_corner.x <= window_size.x && far_corner.y <= window_size.y {
        camera.viewport = Some(Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}

pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.insert_resource(InterfaceState::default());
        app.insert_resource(ComponentUis::default());

        app.add_systems(PreStartup, components::setup);
        app.add_systems(
            PostUpdate,
            (show_ui, set_camera_viewport)
                .chain()
                .before(EguiSet::ProcessOutput)
                .before(TransformSystem::TransformPropagate),
        );
    }
}
