use avian3d::prelude::*;
use bevy::{audio::SpatialScale, prelude::*};
use rand::Rng;
use std::f32::consts::*;

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource, screens::Screen};

use super::{car_colliders::AllCarColliders, consts::{AIRFRICTIONCOEFFICIENT, CARBODYFRICTION, MINIMALVELOCITYFORAIRFRICTION}};

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
    forward_force: f32,
    driving_direction: Vec3, // This has to be a normalized vector!
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    // TODO: Put this in the right schedule
    app.add_systems(
        Update,
        // TODO: Apply a weak torque correction of the avian instabilities when not wrecked.
        (air_friction, accelerate_cars)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        spawn_test_car
        .run_if(in_state(Screen::Gameplay))
        .in_set(AppSystems::Update)
        .in_set(PausableSystems),
    );
}

fn spawn_test_car(mut commands: Commands, car_assets: Res<CarAssets>, all_car_colliders: Option<Res<AllCarColliders>>, mut finished: Local<bool>) {
    if !*finished {
        if let Some(all_car_colliders) = all_car_colliders {
            commands.spawn(car(&car_assets, &all_car_colliders, Vec3::new(-10., 0.01, 0.), 3. * Vec3::X));
            *finished = true;
        }
    }
}

pub fn car(car_assets: &CarAssets, all_car_colliders: &AllCarColliders, init_pos: Vec3, init_vel: Vec3) -> impl Bundle {
    let rng = &mut rand::thread_rng();

    let car_index = rng.gen_range(0..car_assets.get_scenes().len());
    let scene_handle = car_assets.vehicles[car_index].clone();
    let colliders = &all_car_colliders[car_index];
    let forward_force = 380.;
    (
        Name::new("Car"),
        Car { wrecked: false, forward_force, driving_direction: Vec3::X },
        // Physics
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(FRAC_PI_2),
            scale: Vec3::splat(0.8),
        },
        RigidBody::Dynamic,
        colliders.body.clone(),
        children![
            colliders.get_wheel_bl_bundle(),
            colliders.get_wheel_br_bundle(),
            colliders.get_wheel_fl_bundle(),
            colliders.get_wheel_fr_bundle(),
        ],
        LinearVelocity::from(init_vel),
        ExternalForce::default().with_persistence(false),
        Friction::new(CARBODYFRICTION),
        // Gfx and audio
        SceneRoot(scene_handle),
        AudioPlayer::new(car_assets.engine_audio.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_spatial_scale(SpatialScale::new(0.2))
            .with_volume(bevy::audio::Volume::Decibels(-24.))
            .with_speed(rng.gen_range(0.1..0.8) + (init_vel.x / 100.).abs()),
    )
}

fn air_friction(time: Res<Time>, mut cars_query: Query<(&LinearVelocity, &mut ExternalForce)>) {
    for (velocity, mut applied_force) in cars_query.iter_mut() {
        // Only apply this friction to high enough velocities to avoid vibrations when standing still
        if velocity.length() < MINIMALVELOCITYFORAIRFRICTION {
            continue;
        }
        // Apply a force in the opposite direction of the velocity.
        // This force is proportional to the square of the velocity with the given factor.
        // It has to be weighted with the time step. (If changing the physics clock, this needs a look again).
        // The force is cleared by avian every frame.
        let new_force = applied_force.force()
            - AIRFRICTIONCOEFFICIENT * velocity.0.length() * velocity.0 * time.delta_secs();
        applied_force.set_force(new_force);
    }
}

fn accelerate_cars(time: Res<Time>, mut cars: Query<(&Car, &mut ExternalForce)>) {
    for (car, mut applied_force) in cars.iter_mut() {
        if !car.wrecked {
            // Let the car accelerate in the forward direction of the velocity
            let new_force = applied_force.force() + car.driving_direction * car.forward_force * time.delta_secs();
            applied_force.set_force(new_force);
        }
    }
}


#[derive(Debug, Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CarAssets {
    #[dependency]
    vehicles: Vec<Handle<Scene>>,
    #[dependency]
    engine_audio: Handle<AudioSource>,
}

impl CarAssets {
    pub fn get_scenes(&self) -> &Vec<Handle<Scene>> {
        return &self.vehicles;
    }
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
