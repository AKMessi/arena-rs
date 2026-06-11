use crate::CameraShake;
use crate::enemy::Enemy;
use crate::player::{FireCoolDown, Player, Velocity};
use crate::ui::GameState;
use bevy::prelude::*;
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Component)]
struct Bullet;

#[derive(Component)]
pub struct Particle;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct HighScore {
    pub value: u32,
    pub filepath: &'static str,
}

impl Default for HighScore {
    fn default() -> Self {
        let filepath = "highscore.txt";
        let mut value = 0;

        if let Ok(mut file) = File::open(filepath) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(parsed) = contents.trim().parse::<u32>() {
                    value = parsed;
                }
            }
        }

        HighScore { value, filepath }
    }
}

impl HighScore {
    pub fn save(&self) {
        if let Ok(mut file) = File::create(self.filepath) {
            let _ = write!(file, "{}", self.value);
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(
                Update,
                (
                    bullet_spawner_system,
                    bullet_movement_system,
                    collision_system,
                    player_collision_system,
                    particle_system,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Playing), reset_combat_system);
    }
}

fn reset_combat_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<Entity, With<Bullet>>,
    particle_query: Query<Entity, With<Particle>>,
) {
    for entity in &bullet_query {
        commands.entity(entity).despawn();
    }
    for entity in &particle_query {
        commands.entity(entity).despawn();
    }
    score.0 = 0;
}

fn bullet_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut FireCoolDown), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (player_transform, mut cooldown) = player_query.into_inner();
    cooldown.0.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space) && cooldown.0.finished() {
        cooldown.0.reset();
        commands.spawn((
            Sprite {
                image: asset_server.load("textures/bullet.png"),
                custom_size: Some(Vec2::new(8.0, 16.0)),
                color: Color::srgb(1.0, 1.0, 0.0),
                ..default()
            },
            Transform::from_translation(player_transform.translation),
            Bullet,
        ));

        commands.spawn((
            AudioPlayer(asset_server.load::<AudioSource>("audio/shoot.ogg")),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn bullet_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Bullet>>) {
    let bullet_speed = 700.0;
    for mut transform in &mut query {
        transform.translation.y += bullet_speed * time.delta_secs();
    }
}

fn collision_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut camera_query: Query<&mut CameraShake, With<Camera2d>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (enemy_entity, enemy_transform) in &enemy_query {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 16.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.0 += 100;

                if let Ok(mut shake) = camera_query.single_mut() {
                    shake.stress = (shake.stress + 0.4).min(1.0);
                }

                commands.spawn((
                    AudioPlayer(asset_server.load::<AudioSource>("audio/explode.ogg")),
                    PlaybackSettings::DESPAWN, // Clean garbage collection
                ));
                let mut rng = rand::rng();
                for _ in 0..12 {
                    let angle = rng.random_range(0.0..std::f32::consts::TAU);
                    let speed = rng.random_range(120.0..280.0);
                    let velocity_vector = Vec2::new(angle.cos(), angle.sin()) * speed;

                    commands.spawn((
                        Sprite {
                            custom_size: Some(Vec2::new(6.0, 6.0)),
                            color: Color::srgb(1.0, 0.6, 0.1),
                            ..default()
                        },
                        Transform::from_translation(enemy_transform.translation),
                        Velocity(velocity_vector),
                        Particle,
                    ));
                }

                break;
            }
        }
    }
}

fn player_collision_system(
    mut next_state: ResMut<NextState<GameState>>,
    score: Res<Score>,
    mut highscore: ResMut<HighScore>,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let player_pos = player_transform.translation;
    for enemy_transform in &enemy_query {
        let distance = player_pos.distance(enemy_transform.translation);
        if distance < 24.0 {
            if score.0 > highscore.value {
                highscore.value = score.0;
                highscore.save(); // Hard-flushes out to the local file system
                info!("New Personal Best Saved! Score: {}", highscore.value);
            }

            next_state.set(GameState::GameOver);
            break;
        }
    }
}

fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &Velocity, &mut Sprite), With<Particle>>,
) {
    for (entity, mut transform, velocity, mut sprite) in &mut particle_query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
        if let Some(size) = sprite.custom_size {
            let next_size = size.x - 12.0 * time.delta_secs();
            if next_size <= 0.1 {
                commands.entity(entity).despawn();
            } else {
                sprite.custom_size = Some(Vec2::new(next_size, next_size));
            }
        }
    }
}
