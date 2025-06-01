use avian3d::prelude::{Collider, ExternalForce, LinearVelocity, MaxLinearSpeed, RigidBody};
use bevy::{audio::SpatialScale, prelude::*};
use std::f32::consts::*;

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

use super::movement::ScreenWrap;
use rand::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
    velocity: LinearVelocity,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    app.add_systems(
        FixedUpdate,
        car_velocity
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn car(car_assets: &CarAssets, init_pos: Vec3, init_vel: Vec3) -> impl Bundle {
    let rng = &mut rand::thread_rng();
    (
        Name::new("Car"),
        Car {
            wrecked: false,
            velocity: LinearVelocity(init_vel),
        },
        SceneRoot(car_assets.vehicles.choose(rng).unwrap().clone()),
        ScreenWrap,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity::default(),
        MaxLinearSpeed(init_vel.x.abs()),
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(-FRAC_PI_2),
            scale: Vec3::splat(0.8),
        },
        AudioPlayer::new(car_assets.engine_audio.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_spatial_scale(SpatialScale::new(0.2))
            .with_volume(bevy::audio::Volume::Decibels(-24.))
            .with_speed(rng.gen_range(0.1..0.8) + (init_vel.x / 100.).abs()),
    )
}

pub fn car_velocity(time: Res<Time>, cars_query: Query<(&Car, &mut LinearVelocity)>) {
    for (car, mut velocity) in cars_query {
        velocity.x += car.velocity.x * time.delta_secs();
        velocity.x = velocity.x.min(car.velocity.x);
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
