use bevy::prelude::*;
use bevy::core::FixedTimestep;
use crate::{App, components, systems};

pub struct FoodPlugin;

impl Plugin for FoodPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(systems::food_spawner));
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(systems::snake_growth.after(systems::snake_eating))
                .with_system(systems::snake_movement)
                .with_system(systems::snake_eating.after(systems::snake_movement)));
        app.add_system(systems::snake_movement_input.before(systems::snake_movement));
    }
}

pub struct TransformPlugin;

impl Plugin for TransformPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(systems::position_translation)
                .with_system(systems::size_scaling));
    }
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor{
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,..default()})
            .insert_resource(components::LastTailPosition::default())
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .insert_resource(components::SnakeSegments::default())
            .add_startup_system(systems::setup_camera)
            .add_startup_system(systems::spawn_snake)
            .add_event::<components::GameOverEvent>()
            .add_event::<components::GrowthEvent>();
    }
}