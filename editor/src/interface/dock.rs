mod entities;
mod inspector;
mod view;

use bevy::prelude::*;
use bevy_egui::egui;
use egui_dock::TabViewer;

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
}

impl<'a> TabViewer for InterfaceTabViewer<'a> {
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
}
