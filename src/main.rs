use bevy::{asset::UnapprovedPathMode, prelude::*};
use bevy_third_person_camera::*;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

const GRAVITY: f32 = -9.8;
const JUMP_SPEED: f32 = 5.0;
const GROUND_Y: f32 = 0.0;
const SPRINT_SPEED: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            unapproved_path_mode: UnapprovedPathMode::Allow,
            ..default()
        }))
        //.add_plugins(CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        //.add_systems(Update, make_materials_double_sided)
        .run();
}

// Setup a simple 3d scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn girl model as the player
    commands.spawn((
        SceneRoot(asset_server.load("ps1_character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        ThirdPersonCameraTarget,
        Velocity::default(),
    ));

    // spawn camera
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            aim_enabled: true,
            aim_speed: 20.0,
            aim_zoom: 0.7,
            offset_enabled: true,
            offset_toggle_enabled: false,
            gamepad_settings: CustomGamepadSettings { ..default() },
            zoom_enabled: true,
            zoom: Zoom::new(10.0, 60.0),
            sensitivity: Vec2::new(3.0, 3.0),
            cursor_lock_key: KeyCode::Escape,
            ..default()
        },
    ));

    // spawn player
    /*
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ThirdPersonCameraTarget,
        Player,
    ));
    */

    // PS1 scene
    commands.spawn((
        SceneRoot(asset_server.load("ps1_city.glb#Scene0")),
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
    player: Single<(&mut Transform, &mut Velocity), (With<Player>, Without<ThirdPersonCamera>)>,
    camera: Single<&Transform, (With<ThirdPersonCamera>, Without<Player>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    let dt = time.delta_secs();

    // Extract only the horizontal yaw from the camera — immune to steep pitch angles
    let (cam_yaw, _, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let forward = Vec3::new(-cam_yaw.sin(), 0.0, -cam_yaw.cos());
    let right = Vec3::new(cam_yaw.cos(), 0.0, -cam_yaw.sin());

    // W/S: forward/backward, A/D: strafe, relative to camera facing
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if input.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if input.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    if input.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if input.pressed(KeyCode::KeyQ) {
        direction -= right;
    }
    if input.pressed(KeyCode::KeyE) {
        direction += right;
    }

    // Always face the same horizontal direction as the camera (player faces away from camera)
    transform.rotation = Quat::from_rotation_y(cam_yaw + std::f32::consts::PI);

    if direction.length_squared() > 0.001 {
        transform.translation += direction.normalize() * SPRINT_SPEED * dt;
    }

    // Jump and gravity
    let on_ground = transform.translation.y <= GROUND_Y;
    if input.just_pressed(KeyCode::Space) && on_ground {
        velocity.0.y = JUMP_SPEED;
    }
    velocity.0.y += GRAVITY * dt;
    transform.translation.y += velocity.0.y * dt;
    if transform.translation.y < GROUND_Y {
        transform.translation.y = GROUND_Y;
        velocity.0.y = 0.0;
    }
}
