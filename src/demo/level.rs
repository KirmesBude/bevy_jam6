//! Spawn the main level.

use avian3d::prelude::{Collider, RigidBody};
use bevy::{math::VectorSpace, prelude::*, window::PrimaryWindow};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

use super::car::{CarAssets, car};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    app.add_systems(
        Update,
        drop_obstacle
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

fn road(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Road"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2 { x: 100.0, y: 100.0 }))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.7, 0.0))),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(100.0, 0.5, 100.0),
    )
}

fn wall(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    z: f32,
) -> impl Bundle {
    (
        Name::new("Wall"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.0, 0.0))),
        Transform::from_xyz(0.0, -5.0, z).with_scale(Vec3::new(100.0, 30.0, 2.0)),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
    )
}

fn obstacle(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    point: Vec3,
) -> impl Bundle {
    (
        Name::new("Obstacle"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 7.0))),
        Transform::from_translation(point).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
    )
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    car_assets: Res<CarAssets>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (PointLight::default(), Transform::from_xyz(0.0, 5.0, -1.0)),
            (road(&mut meshes, &mut materials)),
            (car(&car_assets)),
            (
                car(&car_assets),
                Transform::from_translation(Vec3 {
                    x: 2.0,
                    y: 0.0,
                    z: -1.0
                })
            ),
            (
                car(&car_assets),
                Transform::from_translation(Vec3 {
                    x: -2.0,
                    y: 0.0,
                    z: 1.0
                })
            ),
            (wall(&mut meshes, &mut materials, 4.0),),
            (wall(&mut meshes, &mut materials, -4.0),),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
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
    if buttons.just_pressed(MouseButton::Right) {
        commands.spawn((
            obstacle(&mut meshes, &mut materials, point),
            StateScoped(Screen::Gameplay),
        ));
    }
}
