use bevy::prelude::*;
use crate::player::{Player, FireCoolDown};
use crate::enemy::Enemy;
use crate::ui::GameState;

#[derive(Component)]
struct Bullet;

#[derive(Resource, Default)]
struct Score(u32);

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
           .add_systems(
               Update,
               (bullet_spawner_system, bullet_movement_system, collision_system, player_collision_system)
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