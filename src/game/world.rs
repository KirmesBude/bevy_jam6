use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::css::GREEN, pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::screens::Screen;

use super::pertubator::spawn_pertubator;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_grass);
    app.add_systems(OnEnter(Screen::Gameplay), spawn_light);
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DirectionalLight {
            illuminance: 1.5 * light_consts::lux::OVERCAST_DAY,
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

// TODO: Add the missing derives
#[derive(Component)]
pub struct Ground;

const GRASS_SIZE: Vec2 = Vec2::new(150., 100.);

fn grass(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Grass"),
        Ground,
        Transform::from_xyz(0., -0.1, 0.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, GRASS_SIZE).mesh())),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        Friction::new(0.05),
        Pickable::default(),
    )
}

pub fn spawn_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            StateScoped(Screen::Gameplay),
            grass(&mut meshes, &mut materials),
        ))
        .observe(spawn_pertubator);
}
