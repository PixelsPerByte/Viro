use bevy::{
    core_pipeline::{bloom::BloomSettings, dof::DepthOfFieldSettings, motion_blur::MotionBlur},
    pbr::{ScreenSpaceAmbientOcclusionSettings, ScreenSpaceReflectionsSettings},
    prelude::*,
};
use bevy_egui::egui;

use crate::{camera::Flycam, EditorEntity};

pub fn settings(world: &mut World, ui: &mut egui::Ui) {
    let Ok(camera) = world
        .query_filtered::<Entity, (With<EditorEntity>, With<Flycam>)>()
        .get_single(world)
    else {
        return;
    };

    let mut entity = world.get_entity_mut(camera).unwrap();

    ui.heading("Graphics");

    ui.checkbox(&mut entity.contains::<BloomSettings>(), "Bloom");
    ui.checkbox(
        &mut entity.contains::<ScreenSpaceAmbientOcclusionSettings>(),
        "SSAO",
    );
    ui.checkbox(
        &mut entity.contains::<ScreenSpaceReflectionsSettings>(),
        "SSR",
    );
    ui.checkbox(
        &mut entity.contains::<DepthOfFieldSettings>(),
        "Depth Of Field",
    );
    ui.checkbox(&mut entity.contains::<MotionBlur>(), "Motion Blur");
}
