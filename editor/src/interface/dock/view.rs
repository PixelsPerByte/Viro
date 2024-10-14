use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        dof::DepthOfFieldSettings,
        motion_blur::MotionBlur,
        prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
    },
    pbr::{
        ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings,
        ScreenSpaceReflectionsSettings,
    },
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

    let mut has_bloom = entity.contains::<BloomSettings>();
    let mut has_ssao = entity.contains::<ScreenSpaceAmbientOcclusionSettings>();
    let mut has_ssr = entity.contains::<ScreenSpaceReflectionsSettings>();
    let mut has_dof = entity.contains::<DepthOfFieldSettings>();
    let mut has_motionblur = entity.contains::<MotionBlur>();

    if ui.checkbox(&mut has_bloom, "Bloom").clicked() {
        if has_bloom {
            entity.insert(BloomSettings::default());
        } else {
            entity.remove::<BloomSettings>();
        }
    }

    if ui.checkbox(&mut has_ssao, "SSAO").clicked() {
        if has_ssao {
            entity.insert(ScreenSpaceAmbientOcclusionSettings::default());
        } else {
            entity.remove::<ScreenSpaceAmbientOcclusionSettings>();
        }
    }

    if ui.checkbox(&mut has_ssr, "SSR").clicked() {
        if has_ssr {
            entity.insert(ScreenSpaceReflectionsSettings::default());
        } else {
            entity.remove::<ScreenSpaceReflectionsSettings>();
        }
    }

    if ui.checkbox(&mut has_dof, "Depth Of Field").clicked() {
        if has_dof {
            entity.insert(DepthOfFieldSettings::default());
        } else {
            entity.remove::<DepthOfFieldSettings>();
        }
    }

    if ui.checkbox(&mut has_motionblur, "Motion Blur").clicked() {
        if has_motionblur {
            entity.insert(MotionBlur::default());
        } else {
            entity.remove::<MotionBlur>();
        }
    }

    // Prepasses
    let has_depth = entity.contains::<DepthPrepass>();
    let has_normal = entity.contains::<NormalPrepass>();
    let has_motion = entity.contains::<MotionVectorPrepass>();
    let has_deferred = entity.contains::<DeferredPrepass>();

    if !has_depth && (has_ssao || has_ssr || has_motionblur) {
        entity.insert(DepthPrepass);
    }

    if !has_normal && (has_ssao) {
        entity.insert(NormalPrepass);
    }

    if !has_motion && (has_motionblur) {
        entity.insert(MotionVectorPrepass);
    }

    if !has_deferred && (has_ssr) {
        entity.insert(DeferredPrepass);
    }
}
