use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (camera_update, camera_follow));
    }
}

pub enum CharacterName {
    One,
    Two,
    Three,
    Debug,
}

// marker
#[derive(Component)]
pub struct Player(CharacterName);

#[derive(Component, Default)]
struct PlayerCamera {
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
struct PlayerData {
    velocity: Vec3,
}

fn spawn_player(mut commands: Commands) {
    // put constants here later
    const PLAYER_FEET: Vec3 = Vec3::ZERO;
    const PLAYER_HEIGHT: Vec3 = Vec3::new(0.0, 1.8, 0.0);
    const PLAYER_RADIUS: f32 = 0.5;
    const OFFSET: f32 = 0.01;
    const MAX_SLOPE_SLIDE: f32 = 45_f32.to_radians();
    const MIN_SLOPE_SLIDE: f32 = 30_f32.to_radians();
    const STEP_CLIMB_HEIGHT: f32 = 0.5;
    const STEP_CLIMB_WIDTH: f32 = 0.2;

    const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 2.0, 10.0);

    // player
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Player(CharacterName::Debug))
        .insert(Collider::capsule(PLAYER_FEET, PLAYER_HEIGHT, PLAYER_RADIUS))
        .insert(Transform::default())
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(OFFSET),
            max_slope_climb_angle: MAX_SLOPE_SLIDE,
            min_slope_slide_angle: MIN_SLOPE_SLIDE,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(STEP_CLIMB_HEIGHT),
                min_width: CharacterLength::Absolute(STEP_CLIMB_WIDTH),
                include_dynamic_bodies: true,
            }),
            ..default()
        });
    commands
        .spawn(PlayerCamera::default())
        .insert(Transform::default())
        .with_children(|parent| {
            parent
                .spawn(Camera3d { ..default() })
                .insert(Transform::from_xyz(
                    CAMERA_POSITION.x,
                    CAMERA_POSITION.y,
                    CAMERA_POSITION.z,
                ));
        });
}

fn camera_update(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut PlayerCamera)>,
    time: Res<Time>,
) {
    const ROTATION_SPEED: f32 = 0.3;
    const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1;

    if let Ok((mut transform, mut orbit)) = query.single_mut() {
        let mut rotation = Vec2::ZERO;
        for event in mouse_motion.read() {
            rotation += event.delta * ROTATION_SPEED * time.delta_secs();
        }

        orbit.yaw -= rotation.x;
        orbit.pitch = (orbit.pitch - rotation.y).clamp(-MAX_PITCH, MAX_PITCH);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, orbit.yaw) * Quat::from_axis_angle(Vec3::X, orbit.pitch);
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<PlayerData>, With<Player>)>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.single(), camera_query.single_mut())
    {
        camera_transform.translation = player_transform.translation;
    }
}

fn player_controls(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut PlayerData,
            &KinematicCharacterControllerOutput,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const WALK: f32 = 5.0;
    const JUMP_FORCE: f32 = 8.0;
    const FRICTION: f32 = 0.875;
    const GRAVITY: f32 = -9.81;

    let Ok(orbit_transform) = camera_query.single() else {
        return;
    };

    if let Ok((mut controller, mut physics, output)) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        let forward = orbit_transform.forward();
        let right = orbit_transform.right();

        let new_forward = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let new_right = Vec3::new(right.x, 0.0, right.z).normalize();

        if input.pressed(KeyCode::KeyW) {
            direction += new_forward;
        }
        if input.pressed(KeyCode::KeyA) {
            direction -= new_right
        }
        if input.pressed(KeyCode::KeyS) {
            direction -= new_forward;
        }
        if input.pressed(KeyCode::KeyD) {
            direction += new_right;
        }

        let mut desired_velocity = if direction != Vec3::ZERO {
            direction.normalize() * WALK
        } else {
            Vec3::ZERO
        };

        desired_velocity.y = physics.velocity.y;

        if input.pressed(KeyCode::Space) {
            physics.velocity.y = JUMP_FORCE;
        }
    }
}
