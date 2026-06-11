mod combat;
mod enemy;
mod player;
mod ui;

use bevy::prelude::*;
use combat::CombatPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "arena-rs".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_camera)
        .add_plugins((PlayerPlugin, EnemyPlugin, CombatPlugin, UiPlugin)) // Grouped registration
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
