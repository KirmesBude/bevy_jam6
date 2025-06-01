use avian3d::prelude::{Collider, LinearVelocity, MaxLinearSpeed, RigidBody};
use bevy::{audio::SpatialScale, prelude::*};
use std::f32::consts::*;

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

use super::level::{ALL_LANES_SPAN_FRAC_2, LANE_NUM, LANE_SPAN};
use rand::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
    velocity: LinearVelocity,
    lane_id: u32,
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

    app.add_systems(
        Update,
        despawn_cars
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn car(car_assets: &CarAssets) -> impl Bundle {
    let rng = &mut rand::thread_rng();

    let lane_idx: u32 = rng.gen_range(0..LANE_NUM) as u32;
    let speed: f32 = rng.gen_range(10..20) as f32;

    (
        Name::new("Car"),
        Car {
            wrecked: false,
            velocity: LinearVelocity(Vec3::new(-speed, 0., 0.)), // remember, -x goes East
            lane_id: lane_idx,
        },
        SceneRoot(car_assets.vehicles.choose(rng).unwrap().clone()),
        // ScreenWrap,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity::default(),
        MaxLinearSpeed(speed),
        Transform {
            translation: Vec3 {
                x: rng.gen_range(32.0..64.0),
                y: 0.,
                z: (ALL_LANES_SPAN_FRAC_2 - (lane_idx as i32 * LANE_SPAN)) as f32,
            },
            rotation: Quat::from_rotation_y(-FRAC_PI_2),
            scale: Vec3::splat(0.8),
        },
        AudioPlayer::new(car_assets.engine_audio.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_spatial_scale(SpatialScale::new(0.2))
            .with_volume(bevy::audio::Volume::Decibels(-24.))
            .with_speed(rng.gen_range(0.1..0.8) + (speed / 100.).abs()),
    )
}

pub fn car_velocity(time: Res<Time>, cars_query: Query<(&Car, &mut LinearVelocity)>) {
    for (car, mut velocity) in cars_query {
        velocity.x += car.velocity.x * time.delta_secs();
        velocity.x = velocity.x.min(car.velocity.x);
    }
}

pub fn despawn_cars(mut commands: Commands, cars_query: Query<(Entity, &Transform), With<Car>>) {
    for (car, transform) in cars_query {
        if transform.translation.x < -12.0 {
            commands.entity(car).despawn();
            info!("Despawning car {:?} {:?}", car, transform);
        }
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
