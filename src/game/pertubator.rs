use avian3d::prelude::{
    Collider, CollisionEventsEnabled, ExternalForce, ExternalImpulse, OnCollisionStart, RigidBody,
    Sensor,
};
use bevy::{color::palettes::css::RED, prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems, screens::Screen};

use super::{car::Car, util::Lifetime};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        drop_obstacle
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(OnEnter(Screen::Gameplay), spawn_spring);
}

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

#[derive(Debug, Component)]
pub struct Spring;

fn spring(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    point: Vec3,
) -> impl Bundle {
    (
        Name::new("Spring"),
        Spring,
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Transform::from_translation(point),
        RigidBody::Static,
        Collider::cylinder(1.0, 1.0),
        Sensor,
        CollisionEventsEnabled,
        Lifetime::new(5.),
    )
}

pub fn spawn_spring(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        spring(&mut meshes, &mut materials, Vec3::new(0.0, 0.0, 0.0)),
    )).observe(|trigger: Trigger<OnCollisionStart>, mut cars: Query<&mut ExternalImpulse, With<Car>>| {
        let spring = trigger.target(); /* TODO: Extract normal from spring for some shenanigans */
        let other_entity = trigger.collider;
        if let Ok(mut impulse) = cars.get_mut(other_entity) {
            dbg!("Car {} triggered spring {}", other_entity, spring);
            impulse.y = 10.0;
        }
    });
}
