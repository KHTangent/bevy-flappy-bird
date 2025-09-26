use std::time::Duration;

use rand::{rng, Rng};

use bevy::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

const GRAVITY_STRENGTH: f32 = 2000.0;
const JUMP_STRENGTH: f32 = 800.0;
const PIPE_SPEED: f32 = 450.0;
const PIPE_GAP: f32 = 225.0;

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const PIPE_WIDTH: f32 = 32.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Pipe;

#[derive(Resource)]
struct PipeSpawnTimer {
	timer: Timer,
}

#[derive(Component, Default)]
#[require(Transform)]
struct Velocity {
	x: f32,
	y: f32,
}

#[derive(Component, Default)]
#[require(Velocity)]
struct Acceleration {
	x: f32,
	y: f32,
}
impl Acceleration {
	fn gravity() -> Self {
		Acceleration {
			x: 0.0,
			y: -GRAVITY_STRENGTH,
		}
	}
}

fn setup(mut commands: Commands) {
	commands.insert_resource(PipeSpawnTimer {
		timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
	});
	commands.spawn(Camera2d);
	commands.spawn((
		Sprite::from_color(Color::srgb(0., 0., 1.), Vec2::ONE),
		Transform {
			translation: Vec3::new(-320.0, 0.0, 0.0),
			scale: PLAYER_SIZE.extend(1.0),
			..default()
		},
		Acceleration::gravity(),
		Velocity::default(),
		Player,
	));
}

fn handle_movement(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		player_velocity.y = JUMP_STRENGTH;
	}
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
	let elapsed = time.delta_secs();
	for (mut transform, velocity) in &mut query {
		let moved = Vec2::new(velocity.x * elapsed, velocity.y * elapsed);
		transform.translation += moved.extend(0.0);
	}
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
	let elapsed = time.delta_secs();
	for (mut velocity, acceleration) in &mut query {
		velocity.x += acceleration.x * elapsed;
		velocity.y += acceleration.y * elapsed;
	}
}

#[derive(Bundle)]
struct PipeBundle {
	sprite: Sprite,
	transform: Transform,
	velocity: Velocity,
	pipe: Pipe,
}

impl PipeBundle {
	fn new(height: f32, y: f32) -> Self {
		PipeBundle {
			sprite: Sprite::from_color(Color::srgb(0., 1., 0.), Vec2::ONE),
			transform: Transform {
				translation: Vec3::new(WINDOW_SIZE.x / 2.0, y - height / 2.0, 0.0),
				scale: Vec3 {
					x: PIPE_WIDTH,
					y: height,
					z: 1.0,
				},
				..default()
			},
			velocity: Velocity {
				x: -PIPE_SPEED,
				y: 0.0,
			},
			pipe: Pipe,
		}
	}
}

fn handle_pipe_spawn(
	mut commands: Commands,
	time: Res<Time>,
	mut pipe_spawn_timer: ResMut<PipeSpawnTimer>,
) {
	pipe_spawn_timer.timer.tick(time.delta());
	if !pipe_spawn_timer.timer.finished() {
		return;
	}
	let pipe_height = WINDOW_SIZE.y;
	let bottom_pos: f32 =
		rng().random_range((-WINDOW_SIZE.y / 2.0)..(WINDOW_SIZE.y / 2.0 - PIPE_GAP));
	commands.spawn_batch([
		PipeBundle::new(pipe_height, bottom_pos + pipe_height + PIPE_GAP),
		PipeBundle::new(pipe_height, bottom_pos),
	]);
}

fn handle_pipe_despawn(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
	for (entity, transform) in query {
		if transform.translation.x < -WINDOW_SIZE.x {
			commands.entity(entity).despawn();
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Flappy game".into(),
				resizable: false,
				resolution: WINDOW_SIZE.into(),
				..default()
			}),
			..default()
		}))
		.add_systems(Startup, setup)
		.add_systems(
			FixedUpdate,
			(
				apply_acceleration,
				apply_velocity,
				handle_pipe_spawn,
				handle_pipe_despawn,
			),
		)
		.add_systems(Update, handle_movement)
		.run();
}
