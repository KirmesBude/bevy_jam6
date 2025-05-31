//! Spawn the main level.

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

use super::car::car;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
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

fn wall(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    z: f32,
) -> impl Bundle {
    (
        Name::new("Wall"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.0, 0.0))),
        Transform::from_xyz(0.0, -5.0, z).with_scale(Vec3::new(100.0, 10.0, 1.0)),
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
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (PointLight::default(), Transform::from_xyz(0.0, 5.0, -1.0)),
            (
                car(&mut meshes, &mut materials),
            ),
            (
                wall(&mut meshes, &mut materials, 4.0),
            ),
            (
                wall(&mut meshes, &mut materials, -4.0),
            ),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}
