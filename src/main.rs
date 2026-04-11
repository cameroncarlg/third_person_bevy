use bevy::{asset::UnapprovedPathMode, prelude::*};

mod camera;

use crate::camera::{CameraOrbit, CameraPlugin, OrbitTarget};

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

const GRAVITY: f32 = -9.8;
const JUMP_SPEED: f32 = 5.0;
const GROUND_Y: f32 = 0.0;
const SPRINT_SPEED: f32 = 5.0;
const TURN_SPEED: f32 = 2.0; // radians per second

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            unapproved_path_mode: UnapprovedPathMode::Allow,
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, make_materials_double_sided)
        .run();
}

// Setup a simple 3d scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // spawn girl model as the player
    commands.spawn((
        SceneRoot(asset_server.load("girl.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        Velocity::default(),
        OrbitTarget,
    ));

    // PS1 scene
    commands.spawn((
        SceneRoot(asset_server.load("ps1_objects.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Lighting
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn make_materials_double_sided(
    mesh_materials: Query<
        &MeshMaterial3d<StandardMaterial>,
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for handle in &mesh_materials {
        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.double_sided = true;
            mat.cull_mode = None;
        }
    }
}

fn move_player(
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut orbit: ResMut<CameraOrbit>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    let dt = time.delta_secs();
    let prev = transform.translation;

    // A/D: rotate the character and camera together (WoW-style turning)
    if input.pressed(KeyCode::KeyA) {
        orbit.yaw += TURN_SPEED * dt;
    }
    if input.pressed(KeyCode::KeyD) {
        orbit.yaw -= TURN_SPEED * dt;
    }

    // Character always faces directly away from the camera
    transform.rotation = Quat::from_rotation_y(orbit.yaw + std::f32::consts::PI);

    // W/S: forward/backward. Q/E: strafe without turning.
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::KeyQ) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyE) {
        direction.x += 1.0;
    }

    let on_ground = transform.translation.y <= GROUND_Y;
    if input.just_pressed(KeyCode::Space) && on_ground {
        velocity.0.y = JUMP_SPEED;
    }

    // Apply gravity
    velocity.0.y += GRAVITY * dt;
    transform.translation.y += velocity.0.y * dt;

    // Clamp to the ground
    if transform.translation.y < GROUND_Y {
        transform.translation.y = GROUND_Y;
        velocity.0.y = 0.0;
    }

    // Rotate movement direction by camera yaw so W always moves away from the camera
    let yaw = orbit.yaw;
    let camera_relative_dir = Vec3::new(
        direction.x * yaw.cos() + direction.z * yaw.sin(),
        0.0,
        -direction.x * yaw.sin() + direction.z * yaw.cos(),
    );

    if camera_relative_dir.length_squared() > 0.001 {
        transform.translation += camera_relative_dir.normalize() * SPRINT_SPEED * dt;
    }

    if transform.translation != prev {
        let t = transform.translation;
        info!("Player position: ({:.2}, {:.2}, {:.2})", t.x, t.y, t.z);
    }
}
