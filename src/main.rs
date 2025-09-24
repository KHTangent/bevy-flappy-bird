use bevy::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
	commands.spawn((
		Sprite::from_color(Color::srgb(0., 0., 1.), Vec2::ONE),
		Transform {
			translation: Vec3::new(128.0, 128.0, 0.0),
			scale: PLAYER_SIZE.extend(1.0),
			..default()
		},
		Player,
	));
}

fn handle_movement(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut player_transform: Single<&mut Transform, With<Player>>,
	time: Res<Time>,
) {
	let elapsed = time.delta_secs();
	let mut direction = Vec2::ZERO;
	if keyboard_input.pressed(KeyCode::ArrowUp) {
		direction.y += 1.0;
	}
	if keyboard_input.pressed(KeyCode::ArrowDown) {
		direction.y -= 1.0;
	}
	if keyboard_input.pressed(KeyCode::ArrowLeft) {
		direction.x -= 1.0;
	}
	if keyboard_input.pressed(KeyCode::ArrowRight) {
		direction.x += 1.0;
	}
	direction = direction.normalize_or_zero() * (elapsed * PLAYER_SPEED);

	player_transform.translation += direction.extend(1.0);
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
		.add_systems(FixedUpdate, handle_movement)
		.run();
}
