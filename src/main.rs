use bevy::core::FixedTimestep;
use bevy::prelude::*;
mod components;
mod systems;
mod plugins;
mod cons;


fn main() {
    App::new()
        .insert_resource(components::LastTailPosition::default())
        .add_event::<components::GameOverEvent>()
        .add_event::<components::GrowthEvent>()
        .insert_resource(WindowDescriptor{
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .add_system(systems::game_over.after(systems::snake_movement))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(components::SnakeSegments::default())
        .add_startup_system(systems::setup_camera)
        .add_startup_system(systems::spawn_snake)
        .add_system(systems::snake_movement_input.before(systems::snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(systems::food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(systems::snake_growth.after(systems::snake_eating))
                .with_system(systems::snake_movement)
                .with_system(systems::snake_eating.after(systems::snake_movement))
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(systems::position_translation)
                .with_system(systems::size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}