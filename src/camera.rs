use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(100.0, 50000.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            far: 10_000.0,
            ..default()
        }),
    ));
}
