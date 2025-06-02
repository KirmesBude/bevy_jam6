use avian3d::prelude::*;
use bevy::{audio::SpatialScale, prelude::*};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::*;

use crate::{asset_tracking::LoadResource, screens::Screen, AppSystems, PausableSystems};

use super::consts::AIRFRICTIONCOEFFICIENT;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    // TODO: Put this in the right schedule
    app.add_systems(
        Update,
        air_friction
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(OnEnter(Screen::Gameplay),
    |mut commands: Commands, car_assets: Res<CarAssets>| {
        commands.spawn(car(&car_assets, Vec3::Y, 3.*Vec3::X));
    });
}

pub fn car(car_assets: &CarAssets, init_pos: Vec3, init_vel: Vec3) -> impl Bundle {
    let rng = &mut rand::thread_rng();
    (
        Name::new("Car"),
        Car {
            wrecked: false,
        },
        // Physics
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(FRAC_PI_2),
            scale: Vec3::splat(0.8),
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 2.0),
        LinearVelocity::from(init_vel),
        ExternalForce::default().with_persistence(false),
        Friction::new(0.5), // Transfer this to the wheels.
        // Gfx and audio
        SceneRoot(car_assets.vehicles.choose(rng).unwrap().clone()),
        AudioPlayer::new(car_assets.engine_audio.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_spatial_scale(SpatialScale::new(0.2))
            .with_volume(bevy::audio::Volume::Decibels(-24.))
            .with_speed(rng.gen_range(0.1..0.8) + (init_vel.x / 100.).abs()),
    )
}

pub fn air_friction(time: Res<Time>, cars_query: Query<(&LinearVelocity, &mut ExternalForce)>) {
    for (velocity, mut applied_force) in cars_query {
        // Apply a force in the opposite direction of the velocity.
        // This force is proportional to the square of the velocity with the given factor.
        // It has to be weighted with the time step. (If changing the physics clock, this needs a look again).
        // The force is cleared by avian every frame.
        let new_force = applied_force.force() - AIRFRICTIONCOEFFICIENT * velocity.0.length() * velocity.0 * time.delta_secs();
        applied_force.set_force(new_force);
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CarAssets {
    #[dependency]
    vehicles: Vec<Handle<Scene>>,
    #[dependency]
    engine_audio: Handle<AudioSource>,
}
const CAR_MODELS: &[&str] = &[
    "ambulance",
    "delivery",
    "delivery-flat",
    "firetruck",
    "garbage-truck",
    "hatchback-sports",
    "police",
    "race",
    "race-future",
    "sedan",
    "sedan-sports",
    "suv",
    "suv-luxury",
    "taxi",
    "tractor",
    "tractor-police",
    "tractor-shovel",
    "truck",
    "truck-flat",
    "van",
];

impl FromWorld for CarAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            vehicles: CAR_MODELS
                .iter()
                .map(|model| {
                    assets.load(
                        GltfAssetLabel::Scene(0)
                            .from_asset(format!("models/car_kit/vehicles/{}.glb", model)),
                    )
                })
                .collect(),
            engine_audio: assets.load("audio/sound_effects/engine-loop.ogg"),
        }
    }
}
