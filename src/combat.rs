use bevy::prelude::*;
use rand::Rng;
use crate::player::{FireCoolDown, Player, Velocity};
use crate::enemy::Enemy;
use crate::ui::GameState;
use crate::CameraShake;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
}

#[derive(Resource, Default)]
struct Score(u32);

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
           .add_systems(
               Update,
               (bullet_spawner_system, 
                   bullet_movement_system, 
                   collision_system, 
                   player_collision_system,
                   particle_system,
                )
                   .run_if(in_state(GameState::Playing)),
           );
    }
}

fn bullet_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut FireCoolDown), With<Player>>,
) {
    let (player_transform, mut cooldown) = player_query.into_inner();
    cooldown.0.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space) && cooldown.0.finished() {
        cooldown.0.reset();
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(8.0, 16.0)),
                color: Color::srgb(1.0, 1.0, 0.0),
                ..default()
            },
            Transform::from_translation(player_transform.translation),
            Bullet,
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
    enemy_query: Query<(Entity, &Transform), With<Enemy>>
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (enemy_entity, enemy_transform) in &enemy_query {
            let distance = bullet_transform.translation.distance(enemy_transform.translation);
            if distance < 16.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.0 += 100;

                if let Ok(mut shake) = camera_query.single_mut() {
                    shake.stress = (shake.stress + 0.4).min(1.0);
                }
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
                        Particle {
                            lifetime: Timer::from_seconds(rng.random_range(0.2..0.5), TimerMode::Once)
                        },
                    ));
                }
                
                break;
            }
        }
    }
}

fn player_collision_system(
    mut next_state: ResMut<NextState<GameState>>,
    player_transform: Single<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let player_pos = player_transform.translation;
    for enemy_transform in &enemy_query {
        let distance = player_pos.distance(enemy_transform.translation);
        if distance < 24.0 {
            next_state.set(GameState::GameOver);
            break;
        }
    }
}

fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &Velocity, &mut Particle, &mut Sprite)>,
) {
    for (entity, mut transform, velocity, mut particle, mut sprite) in &mut particle_query {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation += velocity.0.extend(0.0) * time.delta_secs();
            let life_percentage = 1.0 - particle.lifetime.fraction();
            let new_scale = 6.0 * life_percentage;
            sprite.custom_size = Some(Vec2::new(new_scale, new_scale));
        }
    }
}