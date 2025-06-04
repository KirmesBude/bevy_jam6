use avian3d::prelude::*;
use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_grass);
    app.add_systems(OnEnter(Screen::Gameplay), spawn_light);
}

fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 600.,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            color: Color::linear_rgb(1., 1., 0.8),
            illuminance: 2_500.,
            shadows_enabled: true,
            ..default()
        },
        StateScoped(Screen::Gameplay),
        Transform::from_xyz(1., 6., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// TODO: Add the missing derives
#[derive(Component)]
pub struct Ground;

const GRASS_SIZE: Vec2 = Vec2::new(1000., 1000.);

fn grass(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Grass"),
        Ground,
        Transform::from_xyz(0., -1., 0.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, GRASS_SIZE).mesh())),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        RigidBody::Static,
        Collider::cuboid(GRASS_SIZE.x, 1., GRASS_SIZE.y),
        Friction::new(0.05),
    )
}

pub fn spawn_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        grass(&mut meshes, &mut materials),
    ));
}
