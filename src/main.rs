use bevy::prelude::*;
use rand::Rng;

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
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                player_input_system,
                movement_system,
                bullet_spawner_system,
                bullet_movement_system,
                enemy_spawner_system,
                enemy_movement_system,
                collision_system,
                player_collision_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .init_state::<GameState>()
        .init_resource::<Score>()
        
        .add_systems(OnEnter(GameState::GameOver), game_over_setup_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct FireCoolDown(Timer);

#[derive(Resource, Default)]
struct Score(u32);

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

fn bullet_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &mut FireCoolDown), With<Player>>,
) {
    let  (player_transform, mut cooldown) = player_query.into_inner();
    cooldown.0.tick(time.delta());
    
    if keyboard_input.just_pressed(KeyCode::Space) && cooldown.0.finished() {
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
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    let enemy_speed = 150.0;
    let player_pos = player_transform.translation.xy();

    for mut transform in &mut enemy_query {
        let enemy_pos = transform.translation.xy();
        let direction = (player_pos - enemy_pos).normalize_or_zero();
        transform.translation += direction.extend(0.0) * enemy_speed * time.delta_secs();
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
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

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
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_pos = player_transform.translation;

    for enemy_transform in &enemy_query {
        let distance = player_pos.distance(enemy_transform.1.translation);

        if distance < 24.0 {
            next_state.set(GameState::GameOver);
            break;
        }
    }
}

fn game_over_setup_system(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
        });
}