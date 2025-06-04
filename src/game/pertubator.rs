use avian3d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        drop_obstacle
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Soaped;

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Nailed;

fn obstacle(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    point: Vec3,
) -> impl Bundle {
    (
        Name::new("Obstacle"),
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
        Transform::from_translation(point).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        RigidBody::Dynamic,
        ExternalForce::default().with_persistence(false),
        Collider::cuboid(1.0, 1.0, 1.0),
    )
}

pub fn drop_obstacle(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (camera, cam_transform) = *camera;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let point = ray.get_point(distance);

    /* TODO: Right, because Left triggers on transition */
    if buttons.pressed(MouseButton::Right) {
        commands.spawn((
            obstacle(
                &mut meshes,
                &mut materials,
                Vec3 {
                    x: point.x,
                    y: 5.,
                    z: point.z,
                },
            ),
            StateScoped(Screen::Gameplay),
        ));
    }
}
