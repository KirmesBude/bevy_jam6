use bevy::{prelude::*, render::camera::ScalingMode};

pub(super) fn plugin(app: &mut App) {
    // Spawn and update the zoom of the game camera
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: 100.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 150.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
    ));
}
