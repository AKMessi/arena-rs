use bevy::prelude::*;

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
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                player_input_system,
                movement_system,
                bullet_spawner_system,
                bullet_movement_system,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Bullet;

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            color: Color::WHITE,
            ..default()
        },
        Transform::default(),
        Player,
        Velocity(Vec2::ZERO),
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

fn bullet_spawner_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
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
