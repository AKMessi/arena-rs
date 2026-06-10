use bevy::{math::VectorSpace, prelude::*, render::render_resource::{CachedPipelineState::Ok, Pipeline}};

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
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

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
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut Velocity) = query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        let speed = 400.0;
        velocity.0 = direction.normalize_or_zero() * speed;
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}