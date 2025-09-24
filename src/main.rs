use bevy::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

const GRAVITY_STRENGTH: f32 = 500.0;
const JUMP_STRENGTH: f32 = 500.0;

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
#[require(Transform)]
struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component, Default)]
#[require(Velocity)]
struct Acceleration {
	pub x: f32,
	pub y: f32,
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
	commands.spawn(Camera2d);
	commands.spawn((
		Sprite::from_color(Color::srgb(0., 0., 1.), Vec2::ONE),
		Transform {
			translation: Vec3::new(128.0, 128.0, 0.0),
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
		.add_systems(FixedUpdate, (apply_acceleration, apply_velocity))
		.add_systems(Update, handle_movement)
		.run();
}
