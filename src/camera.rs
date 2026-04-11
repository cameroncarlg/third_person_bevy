use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    color::palettes::basic::SILVER,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
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
        app.init_resource::<CameraOrbit>()
            .add_systems(Startup, initialize_camera)
            .add_systems(Update, camera_follow_system);
    }
}

fn initialize_camera(mut commands: Commands) {
    // This is a custom camera setup
    commands.spawn((
        MainCamera,
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
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

/*
fn camera_movement_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<OrbitTarget>)>,
    orbit_target: Option<Single<&Transform, (With<OrbitTarget>, Without<MainCamera>)>>,
) {
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

    // Scroll wheel: zoom in/out along camera's forward axis
    if mouse_scroll.delta.y != 0.0 {
        let zoom_speed = 5.0;
        let forward = *camera.forward();
        camera.translation += forward * mouse_scroll.delta.y * zoom_speed;
    }
}
*/

#[derive(Resource)]
pub struct CameraOrbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.7, // ~40 degrees above horizontal
            distance: 7.0,
        }
    }
}

fn camera_follow_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut orbit: ResMut<CameraOrbit>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<OrbitTarget>)>,
    target: Option<Single<&Transform, (With<OrbitTarget>, Without<MainCamera>)>>,
) {
    // Right-click drag: rotate orbit angle
    if mouse_input.pressed(MouseButton::Right) && mouse_motion.delta != Vec2::ZERO {
        let sensitivity = 0.005;
        orbit.yaw -= mouse_motion.delta.x * sensitivity;
        const PITCH_MIN: f32 = 0.05;
        const PITCH_MAX: f32 = std::f32::consts::FRAC_PI_2 - 0.05;
        orbit.pitch = (orbit.pitch + mouse_motion.delta.y * sensitivity).clamp(PITCH_MIN, PITCH_MAX);
    }

    // Scroll wheel: zoom in/out
    if mouse_scroll.delta.y != 0.0 {
        orbit.distance = (orbit.distance - mouse_scroll.delta.y * 0.5).clamp(2.0, 20.0);
    }

    // Follow target: recompute camera position every frame
    let target_pos = target.map(|t| t.translation).unwrap_or(Vec3::ZERO);

    let horizontal = orbit.distance * orbit.pitch.cos();
    let offset = Vec3::new(
        horizontal * orbit.yaw.sin(),
        orbit.distance * orbit.pitch.sin(),
        horizontal * orbit.yaw.cos(),
    );

    camera.translation = target_pos + offset;
    camera.look_at(target_pos, Vec3::Y);
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
