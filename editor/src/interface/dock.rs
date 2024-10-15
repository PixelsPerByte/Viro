mod entities;
mod inspector;
mod view;

use bevy::prelude::*;
use bevy_egui::egui;
use egui_dock::TabViewer;

pub struct AddTab {
    pub tab: InterfaceTab,
    pub surface: egui_dock::SurfaceIndex,
    pub node: egui_dock::NodeIndex,
}

pub enum InterfaceTab {
    Viewport,
    Entities,
    Inspector,
    ViewSettings,
}

pub struct InterfaceTabViewer<'a> {
    pub world: &'a mut World,
    pub viewport_rect: &'a mut egui::Rect,
    pub cursor_over_ui: &'a mut bool,
    pub added_tabs: &'a mut Vec<AddTab>,
}

impl TabViewer for InterfaceTabViewer<'_> {
    type Tab = InterfaceTab;

    fn title(&mut self, tab: &mut Self::Tab) -> bevy_egui::egui::WidgetText {
        match tab {
            InterfaceTab::Viewport => "Viewport".into(),
            InterfaceTab::Entities => "Entities".into(),
            InterfaceTab::Inspector => "Inspector".into(),
            InterfaceTab::ViewSettings => "View Settings".into(),
        }
    }

    fn ui(&mut self, ui: &mut bevy_egui::egui::Ui, tab: &mut Self::Tab) {
        match tab {
            InterfaceTab::Viewport => {
                *self.cursor_over_ui = !ui.rect_contains_pointer(ui.clip_rect());
                *self.viewport_rect = ui.clip_rect();

                // FPS
                let time = self.world.resource::<Time<Virtual>>();
                let fps = 1.0 / time.delta_seconds();
                let trunc_fps = (fps * 100.0).trunc() / 100.0;

                ui.label(
                    egui::RichText::new(format!("fps: {}", trunc_fps)).color(egui::Color32::WHITE),
                );
            }
            InterfaceTab::Entities => {
                entities::show(self.world, ui);
            }
            InterfaceTab::Inspector => {
                inspector::show(self.world, ui);
            }
            InterfaceTab::ViewSettings => {
                view::settings(self.world, ui);
            }
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, InterfaceTab::Viewport)
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, InterfaceTab::Viewport)
    }

    fn add_popup(
        &mut self,
        ui: &mut egui::Ui,
        surface: egui_dock::SurfaceIndex,
        node: egui_dock::NodeIndex,
    ) {
        ui.set_min_width(120.0);
        ui.style_mut().visuals.button_frame = false;

        let tab = if ui.button("Entities").clicked() {
            InterfaceTab::Entities
        } else if ui.button("Inspector").clicked() {
            InterfaceTab::Inspector
        } else if ui.button("View Settings").clicked() {
            InterfaceTab::ViewSettings
        } else {
            return;
        };

        self.added_tabs.push(AddTab { tab, surface, node });
    }
}
