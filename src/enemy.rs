use bevy::prelude::*;
use rand::Rng;
use crate::player::Player;
use crate::ui::GameState;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource, Default)]
pub struct WaveManager {
    pub elapsed_secs: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(
                Update, 
                (   enemy_spawner_system, 
                    enemy_movement_system,
                    wave_difficulty_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn wave_difficulty_system(
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    wave_manager.elapsed_secs += time.delta_secs();

    let new_duration = (1.0 - (wave_manager.elapsed_secs * 0.01)).clamp(0.15, 1.0);
    spawn_timer.0.set_duration(Duration::from_secs_f32(new_duration));
}

fn enemy_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let edge = rng.random_range(0..4);

        let mut spawn_pos = Vec2::ZERO;
        match edge {
            0 => {
                spawn_pos.x = rng.random_range(-400.0..400.0);
                spawn_pos.y = 300.0;
            }
            1 => {
                spawn_pos.x = rng.random_range(-400.0..400.0);
                spawn_pos.y = -300.0;
            }
            2 => {
                spawn_pos.x = -400.0;
                spawn_pos.y = rng.random_range(-300.0..300.0);
            }
            _ => {
                spawn_pos.x = 400.0;
                spawn_pos.y = rng.random_range(-300.0..300.0);
            }
        }

        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(24.0, 24.0)),
                color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            },
            Transform::from_translation(spawn_pos.extend(0.0)),
            Enemy,
        ));
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    wave_manager: Res<WaveManager>,
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    let enemy_speed = 150.0 + (wave_manager.elapsed_secs * 2.0).min(170.0);
    let player_pos = player_transform.translation.xy();

    for mut transform in &mut enemy_query {
        let enemy_pos = transform.translation.xy();
        let direction = (player_pos - enemy_pos).normalize_or_zero();
        transform.translation += direction.extend(0.0) * enemy_speed * time.delta_secs();
    }
}