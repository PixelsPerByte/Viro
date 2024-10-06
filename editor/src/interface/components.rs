use bevy::{prelude::*, utils::HashMap};
use bevy_egui::egui::{self, ComboBox, DragValue};
use std::any::TypeId;

#[derive(Resource, Default)]
pub struct ComponentUis(pub HashMap<TypeId, Box<dyn Fn(&mut egui::Ui, EntityWorldMut<'_>)>>);

unsafe impl Send for ComponentUis {}
unsafe impl Sync for ComponentUis {}

pub fn setup(mut component_uis: ResMut<ComponentUis>) {
    component_uis.0.insert(
        TypeId::of::<Name>(),
        Box::new(|ui, mut entity| {
            let mut name = entity.get_mut::<Name>().unwrap();
            name.mutate(|s| {
                ui.text_edit_singleline(s);
            });
        }),
    );
    component_uis.0.insert(
        TypeId::of::<Visibility>(),
        Box::new(|ui, mut entity| {
            let mut visibility = entity.get_mut::<Visibility>().unwrap();

            ComboBox::from_label("Visibility")
                .selected_text(match *visibility {
                    Visibility::Inherited => "Inherited",
                    Visibility::Hidden => "Hidden",
                    Visibility::Visible => "Visible",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *visibility, Visibility::Inherited, "Inherited");
                    ui.selectable_value(&mut *visibility, Visibility::Hidden, "Hidden");
                    ui.selectable_value(&mut *visibility, Visibility::Visible, "Visible");
                });
        }),
    );
    component_uis.0.insert(
        TypeId::of::<Transform>(),
        Box::new(|ui, mut entity| {
            let mut transform = entity.get_mut::<Transform>().unwrap();

            ui.label("Translation");
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut transform.translation.x).prefix("X "));
                ui.add(DragValue::new(&mut transform.translation.y).prefix("Y "));
                ui.add(DragValue::new(&mut transform.translation.z).prefix("Z "));
            });

            ui.label("Rotation");
            ui.horizontal(|ui| {
                let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);
                let (mut rx, mut ry, mut rz) = (rx.to_degrees(), ry.to_degrees(), rz.to_degrees());
                ui.add(DragValue::new(&mut rx).prefix("X "));
                ui.add(DragValue::new(&mut ry).prefix("Y "));
                ui.add(DragValue::new(&mut rz).prefix("Z "));

                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    rx.to_radians(),
                    ry.to_radians(),
                    rz.to_radians(),
                );
            });

            ui.label("Scale");
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut transform.scale.x).prefix("X "));
                ui.add(DragValue::new(&mut transform.scale.y).prefix("Y "));
                ui.add(DragValue::new(&mut transform.scale.z).prefix("Z "));
            });
        }),
    );
}
