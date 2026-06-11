mod combat;
mod enemy;
mod player;
mod ui;

use bevy::prelude::*;
use rand::Rng;
use combat::CombatPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

#[derive(Component, Default)]
pub struct CameraShake {
    pub stress: f32,
}

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
        .add_systems(Update, camera_shake_system)
        .add_plugins((PlayerPlugin, EnemyPlugin, CombatPlugin, UiPlugin)) // Grouped registration
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_shake_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut CameraShake), With<Camera2d>>,
) {
    if let Ok((mut transform, mut shake)) = camera_query.single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;

        if shake.stress > 0.0 {
            shake.stress = (shake.stress - time.delta_secs() * 2.5).max(0.0);

            let mut rng = rand::rng();

            let shake_intensity = shake.stress * shake.stress;
            let max_translation_offset = 12.0;

            transform.translation.x += rng.random_range(-1.0..1.0) * max_translation_offset * shake_intensity;
            transform.translation.y += rng.random_range(-1.0..1.0) * max_translation_offset * shake_intensity;
        }
    }
}
