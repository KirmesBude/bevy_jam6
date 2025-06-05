use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::screens::Screen;

const ZOOM_SCROLL_FACTOR: f32 = 256.;

pub(super) fn plugin(app: &mut App) {
    // Spawn and update the zoom of the game camera
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, zoom_camera.run_if(in_state(Screen::Gameplay)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 32.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 180.0, 70.0).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
    ));
}

fn zoom_camera(
    projection: Single<&mut Projection>,
    time: Res<Time>,
    acc_scroll: Res<AccumulatedMouseScroll>,
) {
    let scroll_y = match acc_scroll.unit {
        MouseScrollUnit::Pixel => acc_scroll.delta.y / 100.0,
        MouseScrollUnit::Line => acc_scroll.delta.y,
    };

    let delta = scroll_y * ZOOM_SCROLL_FACTOR * time.delta_secs();

    if delta == 0.0 {
        return;
    }

    if let Projection::Orthographic(ref mut ortho) = *projection.into_inner() {
        if let ScalingMode::FixedVertical { viewport_height } = &mut ortho.scaling_mode {
            let autoscale_factor = 1. - (1.0 / (1. + *viewport_height));
            *viewport_height += delta * autoscale_factor;
            *viewport_height = viewport_height.clamp(8., 128.);

            // info!(viewport_height, delta, scroll_y, acc_scroll.delta.y);
        }
    }
}
