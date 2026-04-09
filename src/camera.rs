use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    color::palettes::basic::SILVER,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

pub struct CameraPlugin;

#[derive(Component)]
#[require(Camera3d, FreeCamera)]
pub struct MainCamera;

/// Attach this to any entity the camera should orbit around with Q/E.
#[derive(Component)]
pub struct OrbitTarget;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera)
            .add_systems(Update, camera_movement_system);
    }
}

fn initialize_camera(mut commands: Commands) {
    // This is a custom camera setup
    commands.spawn((
        MainCamera,
        //Transform::from_xyz(100.0, 50000.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        //Projection::Perspective(PerspectiveProjection {
        //    far: 10_000.0,
        //    ..default()
        //}),
    ));

    // This is the default free camera setup from bevy
    /*
    commands.spawn((
        MainCamera,
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
    ));
    */
}

/*
fn rotate_camera_to_mouse(
    time: Res<Time>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut transform: Single<&mut Transform, With<Camera>>,
) {
    let dt = time.delta_secs();
    let mouse_sensitivity = Vec2::new(0.12, 0.10);

    for motion in mouse_motion.read() {
        let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
        let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

        // Add yaw which is turning left/right (global)
        transform.rotate_y(delta_yaw);

        // Add pitch which is looking up/down
        const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        // Apply the rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
*/

fn camera_movement_system(
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<OrbitTarget>)>,
    orbit_target: Option<Single<&Transform, (With<OrbitTarget>, Without<MainCamera>)>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let move_speed = 10.0;

    let rotate_speed = 2.0;

    // Right-click drag: yaw (left/right) and pitch (up/down) orbit around target
    if mouse_input.pressed(MouseButton::Right) && mouse_motion.delta != Vec2::ZERO {
        let sensitivity = 0.005;
        let delta = mouse_motion.delta;
        let pivot = orbit_target
            .as_ref()
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);

        // Yaw: orbit horizontally around world Y axis
        camera.rotate_around(pivot, Quat::from_rotation_y(-delta.x * sensitivity));

        // Pitch: orbit vertically around camera's local right axis, clamped to avoid poles
        const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.05;
        let current_pitch = camera.rotation.to_euler(EulerRot::YXZ).1;
        let delta_pitch = (current_pitch - delta.y * sensitivity).clamp(-PITCH_LIMIT, PITCH_LIMIT)
            - current_pitch;
        if delta_pitch != 0.0 {
            let right = *camera.right();
            camera.rotate_around(pivot, Quat::from_axis_angle(right, delta_pitch));
        }
    }

    /*
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
    */
}

/*
fn mouse_debug_system(
    mut button_events: EventReader<MouseButtonInput>,
    mut motion_events: EventReader<MouseMotion>,
    mut cursor_events: EventReader<CursorMoved>,
    mut wheel_events: EventReader<MouseWheel>,
    mut pinch_events: EventReader<PinchGesture>,
    mut rotation_events: EventReader<RotationGesture>,
) {
    for event in button_events.read() {
        info!("Mouse button event: {:?}", event);
    }
    for event in motion_events.read() {
        info!("Mouse button event: {:?}", event);
    }
    for event in cursor_events.read() {
        info!("Mouse button event: {:?}", event);
    }
    for event in wheel_events.read() {
        info!("Mouse button event: {:?}", event);
    }
    for event in pinch_events.read() {
        info!("Mouse button event: {:?}", event);
    }
    for event in rotation_events.read() {
        info!("Mouse button event: {:?}", event);
    }
}
*/
