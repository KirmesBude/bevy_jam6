use std::f32::consts::PI;

use bevy::{
    color::palettes::css::ORANGE_RED, pbr::CascadeShadowConfigBuilder, prelude::*,
    render::camera::ScalingMode,
};

pub(super) fn plugin(app: &mut App) {
    // Spawn and update the zoom of the game camera
    app.add_systems(Startup, (spawn_camera, spawn_light));
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

fn spawn_light(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 1.0 * light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI / 2., -PI / 4.)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 200.0,
            ..default()
        }
        .build(),
    ));
}
