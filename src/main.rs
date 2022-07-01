use bevy::prelude::*;

use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
mod systems;
mod plugins;
mod cons;

fn main() {
    App::new()
        .add_plugin(plugins::SetupPlugin)
        .add_plugin(plugins::FoodPlugin)
        .add_plugin(plugins::SnakePlugin)
        .add_plugin(plugins::TransformPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(systems::game_over.after(systems::snake_movement))
        .run();
}