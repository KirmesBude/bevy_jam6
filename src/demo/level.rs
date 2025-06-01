//! Spawn the main level.

use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::{prelude::*, window::PrimaryWindow};
use std::f32::consts::*;

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, audio::music, screens::Screen,
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

    app.add_systems(
        Update,
        spawn_cars
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub const LANE_NUM: i32 = 8;
pub const LANE_SPAN: i32 = 2;
pub const ALL_LANES_SPAN: i32 = LANE_NUM * LANE_SPAN;
pub const ALL_LANES_SPAN_FRAC_2: i32 = ALL_LANES_SPAN / 2;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

#[derive(Component)]
pub struct Level;

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
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        children![GizmoAsset::new().line(
            Vec3 {
                x: -50.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 50.0,
                y: 0.0,
                z: 0.0,
            },
            Color::WHITE,
        ),],
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(200.0, 1., 200.0),
        Friction::new(0.01),
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
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
        Transform::from_xyz(0.0, -5.0, z).with_scale(Vec3::new(100.0, 20.0, 2.0)),
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
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
        Transform::from_translation(point).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        RigidBody::Dynamic,
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
    let rng = rand::thread_rng();

    commands
        .spawn((
            Level,
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                DirectionalLight {
                    illuminance: 2_000.0,
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2 - 0.2)),
            ));
            parent.spawn(road(&mut meshes, &mut materials));
            parent.spawn(wall(&mut meshes, &mut materials, (ALL_LANES_SPAN) as f32));
            // parent.spawn(wall(&mut meshes, &mut materials, -1.0));
            parent.spawn(wall(
                &mut meshes,
                &mut materials,
                (ALL_LANES_SPAN) as f32 * -1.,
            ));
            parent.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.clone()),
            ));

            // for x_offs in (0..2000).step_by(200) {
            //     for lane in 0..LANE_NUM {
            //         let pos = Vec3 {
            //             x: x_offs as f32 + lane as f32 + rng.gen_range(-1.0..1.0),
            //             y: 0.0,
            //             z: ((ALL_LANES_SPAN / 2) - (lane * LANE_SPAN)) as f32,
            //         };
            //         let vel = Vec3 {
            //             // x: -rng.gen_range(20.0..25.0),
            //             x: -40.,
            //             y: 0.,
            //             z: 0.,
            //         };

            //         let ent: EntityCommands<'_> = parent.spawn(car(&car_assets, pos, vel));

            //         // dbg!(ent.id(), pos, vel);
            //     }
            // }
        });
}

fn spawn_cars(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    car_assets: Res<CarAssets>,
    time: Res<Time>,
) {
    // dbg!(&time.elapsed().as_secs_f32() % 1.0);

    if time.elapsed_secs() % 0.1 >= 0.02 {
        return;
    }
    // info!("Spawning car");

    commands.spawn(car(&car_assets));
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
