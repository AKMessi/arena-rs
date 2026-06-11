use bevy::prelude::*;
use crate::ui::GameState;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct FireCoolDown(pub Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system)
            .add_systems(
                Update,
                (player_input_system, movement_system, clamp_bounds_system)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/player.png"),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            color: Color::WHITE,
            ..default()
        },
        Transform::default(),
        Player,
        Velocity(Vec2::ZERO),
        FireCoolDown(Timer::from_seconds(0.2, TimerMode::Once)),
    ));
}

fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
        let mut direction = Vec2::ZERO;
    
        if keyboard_input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
    }

    let speed = 400.0;
    player_velocity.0 = direction.normalize_or_zero() * speed;
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn clamp_bounds_system(mut player_transform: Single<&mut Transform, With<Player>>) {
    player_transform.translation.x = player_transform.translation.x.clamp(-384.0, 384.0);
    player_transform.translation.y = player_transform.translation.y.clamp(-284.0, 284.0);
}