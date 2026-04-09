use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera)
            .add_systems(Update, camera_movement_system);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        //Transform::from_xyz(100.0, 50000.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        //Projection::Perspective(PerspectiveProjection {
        //    far: 10_000.0,
        //    ..default()
        //}),
    ));
}

fn camera_movement_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let move_speed = 10.0;

    let mut direction = Vec3::ZERO;

    // Forward/Backward (W/S)
    if input.pressed(KeyCode::KeyW) {
        direction += *camera.forward();
    }
    if input.pressed(KeyCode::KeyS) {
        direction -= *camera.forward();
    }

    // Left/Right (A/D)
    if input.pressed(KeyCode::KeyA) {
        direction -= *camera.right();
    }
    if input.pressed(KeyCode::KeyD) {
        direction += *camera.right();
    }

    if direction != Vec3::ZERO {
        let direction = direction.normalize();
        camera.translation += direction * move_speed * dt;
    }
}
