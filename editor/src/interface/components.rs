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
                ui.add(
                    DragValue::new(&mut transform.translation.x)
                        .prefix("X ")
                        .speed(0.1),
                );
                ui.add(
                    DragValue::new(&mut transform.translation.y)
                        .prefix("Y ")
                        .speed(0.1),
                );
                ui.add(
                    DragValue::new(&mut transform.translation.z)
                        .prefix("Z ")
                        .speed(0.1),
                );
            });

            ui.label("Rotation");
            ui.horizontal(|ui| {
                let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);
                let (mut rx, mut ry, mut rz) = (rx.to_degrees(), ry.to_degrees(), rz.to_degrees());
                ui.add(DragValue::new(&mut rx).prefix("X ").speed(0.1));
                ui.add(DragValue::new(&mut ry).prefix("Y ").speed(0.1));
                ui.add(DragValue::new(&mut rz).prefix("Z ").speed(0.1));

                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    rx.to_radians(),
                    ry.to_radians(),
                    rz.to_radians(),
                );
            });

            ui.label("Scale");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut transform.scale.x)
                        .prefix("X ")
                        .speed(0.1),
                );
                ui.add(
                    DragValue::new(&mut transform.scale.y)
                        .prefix("Y ")
                        .speed(0.1),
                );
                ui.add(
                    DragValue::new(&mut transform.scale.z)
                        .prefix("Z ")
                        .speed(0.1),
                );
            });
        }),
    );

    component_uis.0.insert(
        TypeId::of::<Handle<StandardMaterial>>(),
        Box::new(|ui, mut entity| {
            let handle = entity
                .get::<Handle<StandardMaterial>>()
                .unwrap()
                .clone_weak();
            let world = unsafe { entity.world_mut() };
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            let material = materials.get_mut(&handle).unwrap();

            ui.horizontal(|ui| {
                let rgba = material.base_color.to_srgba().to_u8_array();
                let mut color =
                    egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                ui.label("Base Color");
                if ui.color_edit_button_srgba(&mut color).changed() {
                    material.base_color = Srgba::from_u8_array(color.to_array()).into();
                }
            });

            ui.horizontal(|ui| {
                let rgba = material.emissive.to_u8_array();
                let mut color =
                    egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                ui.label("Emissive");
                if ui.color_edit_button_srgba(&mut color).changed() {
                    material.emissive = LinearRgba::from_u8_array(color.to_array()).into();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Emissive Exposure Weight");
                ui.add(
                    egui::DragValue::new(&mut material.emissive_exposure_weight)
                        .range(0.0..=1.0)
                        .speed(0.01),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Perceptual Roughness");
                ui.add(
                    egui::DragValue::new(&mut material.perceptual_roughness)
                        .range(0.089..=1.0)
                        .speed(0.01),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Metallic");
                ui.add(
                    egui::DragValue::new(&mut material.metallic)
                        .range(0.0..=1.0)
                        .speed(0.01),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Reflectance");
                ui.add(
                    egui::DragValue::new(&mut material.reflectance)
                        .range(0.0..=1.0)
                        .speed(0.01),
                );
            });
        }),
    );

    component_uis.0.insert(
        TypeId::of::<PointLight>(),
        Box::new(|ui, mut entity| {
            let mut light = entity.get_mut::<PointLight>().unwrap();

            ui.horizontal(|ui| {
                let rgba = light.color.to_srgba().to_u8_array();
                let mut color =
                    egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                ui.label("Color");
                if ui.color_edit_button_srgba(&mut color).changed() {
                    light.color = Srgba::from_u8_array(color.to_array()).into();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Intensity");
                ui.add(egui::DragValue::new(&mut light.intensity).suffix(" lumens"));
            });

            ui.horizontal(|ui| {
                ui.label("Range");
                ui.add(egui::DragValue::new(&mut light.range).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Radius");
                ui.add(egui::DragValue::new(&mut light.radius).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Shadows");
                ui.checkbox(&mut light.shadows_enabled, "");
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Depth Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_depth_bias).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Normal Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_normal_bias).speed(0.01));
            });
        }),
    );
    component_uis.0.insert(
        TypeId::of::<SpotLight>(),
        Box::new(|ui, mut entity| {
            let mut light = entity.get_mut::<SpotLight>().unwrap();

            ui.horizontal(|ui| {
                let rgba = light.color.to_srgba().to_u8_array();
                let mut color =
                    egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                ui.label("Color");
                if ui.color_edit_button_srgba(&mut color).changed() {
                    light.color = Srgba::from_u8_array(color.to_array()).into();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Intensity");
                ui.add(egui::DragValue::new(&mut light.intensity).suffix(" lumens"));
            });

            ui.horizontal(|ui| {
                ui.label("Range");
                ui.add(egui::DragValue::new(&mut light.range).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Radius");
                ui.add(egui::DragValue::new(&mut light.radius).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Outer Angle");
                ui.drag_angle_tau(&mut light.outer_angle);
            });

            ui.horizontal(|ui| {
                ui.label("Inner Angle");
                ui.drag_angle_tau(&mut light.inner_angle);
            });

            ui.horizontal(|ui| {
                ui.label("Shadows");
                ui.checkbox(&mut light.shadows_enabled, "");
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Depth Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_depth_bias).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Normal Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_normal_bias).speed(0.01));
            });
        }),
    );
    component_uis.0.insert(
        TypeId::of::<DirectionalLight>(),
        Box::new(|ui, mut entity| {
            let mut light = entity.get_mut::<DirectionalLight>().unwrap();

            ui.horizontal(|ui| {
                let rgba = light.color.to_srgba().to_u8_array();
                let mut color =
                    egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);

                ui.label("Color");
                if ui.color_edit_button_srgba(&mut color).changed() {
                    light.color = Srgba::from_u8_array(color.to_array()).into();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Illuminance");
                ui.add(egui::DragValue::new(&mut light.illuminance).suffix(" lux"));
            });

            ui.horizontal(|ui| {
                ui.label("Shadows");
                ui.checkbox(&mut light.shadows_enabled, "");
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Depth Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_depth_bias).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Normal Bias");
                ui.add(egui::DragValue::new(&mut light.shadow_normal_bias).speed(0.01));
            });
        }),
    );
}
