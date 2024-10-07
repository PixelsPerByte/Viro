use bevy::{prelude::*, utils::HashMap};

use crate::{
    transform::{cancel_transform, finish_transform, TransformEntities, TransformSelected},
    EditorEntity, SelectedEntities,
};

pub fn setup(mut commands: Commands) {
    commands.spawn((Observer::new(select_entity), EditorEntity));
    commands.spawn((Observer::new(transform_selected), EditorEntity));
    commands.spawn((Observer::new(transform_finish), EditorEntity));
    commands.spawn((Observer::new(transform_cancel), EditorEntity));
}

#[derive(Event)]
pub struct SelectEntity {
    pub target: Entity,
}

pub fn select_entity(trigger: Trigger<SelectEntity>, mut selected: ResMut<SelectedEntities>) {
    let event = trigger.event();

    if !selected.0.remove(&event.target) {
        selected.0.insert(event.target);
    }
}

#[derive(Event)]
pub struct FinishTransform;

#[derive(Event)]
pub struct CancelTransform;

pub fn transform_selected(
    trigger: Trigger<TransformSelected>,
    selected: Res<SelectedEntities>,
    transform_query: Query<&Transform>,
    transform_entities: Option<Res<TransformEntities>>,
    mut commands: Commands,
) {
    if transform_entities.is_some() {
        return;
    }

    if selected.0.is_empty() {
        info!("No entity is selected.");
        return;
    }

    let mut resource = TransformEntities {
        entities: HashMap::new(),
        delta: 0.0,
        axis: Vec3::ZERO, // TODO
        mode: trigger.event().clone(),
        center: Vec3::ZERO,
    };

    for entity in selected.0.iter() {
        let Ok(transform) = transform_query.get(*entity) else {
            info!("The transform operation has been cancelled.");
            warn!("A selected entity doesn't have a `Transform` component.");
            return;
        };

        let data = match trigger.event() {
            TransformSelected::Translate => transform.translation,
            TransformSelected::Rotate => transform.rotation.to_euler(EulerRot::XYZ).into(),
            TransformSelected::Scale => transform.scale,
        };

        resource.entities.insert(*entity, data);
        resource.center += transform.translation;
    }

    resource.center /= selected.0.len() as f32;
    commands.insert_resource(resource);
}

pub fn transform_finish(
    _trigger: Trigger<FinishTransform>,
    transform_entities: Res<TransformEntities>,
    transform_query: Query<&Transform>,
    commands: Commands,
) {
    finish_transform(transform_entities, transform_query, commands);
}

pub fn transform_cancel(
    _trigger: Trigger<CancelTransform>,
    transform_entities: Res<TransformEntities>,
    transform_query: Query<&mut Transform>,
    commands: Commands,
) {
    cancel_transform(transform_entities, transform_query, commands);
}

pub struct ObserverPlugin;
impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}
