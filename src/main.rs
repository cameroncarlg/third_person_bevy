use bevy::{asset::UnapprovedPathMode, prelude::*};

mod camera;

use crate::camera::CameraPlugin;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

const GRAVITY: f32 = -9.8;
const JUMP_SPEED: f32 = 5.0;
const GROUND_Y: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            unapproved_path_mode: UnapprovedPathMode::Allow,
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

//fn initialize_camera(mut commands: Commands) {}

// Setup a simple 3d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn cube, color, assign it as the player, assign it velocity
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.9, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
        Velocity::default(),
    ));

    // Lighting
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    //commands.spawn((
    //    MainCamera,
    //    Transform::from_xyz(100.0, 50000.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    Projection::Perspective(PerspectiveProjection {
    //        far: 10_000.0,
    //        ..default()
    //    }),
    //));
}

/*
fn move_player(
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    //mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    let dt = time.delta_secs();

    // initilize empty direction
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let on_ground = transform.translation.y <= GROUND_Y;
    if input.just_pressed(KeyCode::Space) && on_ground {
        velocity.0.y = JUMP_SPEED;
    }

    // apply gravity
    velocity.0.y += GRAVITY * dt;
    transform.translation.y += velocity.0.y * dt;

    // clamp to the ground
    if transform.translation.y < GROUND_Y {
        transform.translation.y = GROUND_Y;
        velocity.0.y = 0.0;
    }

    let speed = 5.0;
    //let mut transform = query.single_mut();
    transform.translation += direction * speed * time.delta_secs();
}
*/

//fn check_scene_loaded(scenes: Query<&SceneRoot>, asset_server: Res<AssetServer>) {
//    for scene in &scenes {
//        let state = asset_server.get_load_state(scene.0.id());
//        info!("{:?}", state);
//    }
//}
//
//fn load_gltf_things(mut commands: Commands, server: Res<AssetServer>) {
//    commands.spawn(SceneRoot(
//        server.load("uploads_files_2720101_BusGameMap.glb#Scene0"),
//    ));
//}
