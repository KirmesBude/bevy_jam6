use avian3d::prelude::{Collider, ExternalForce, RigidBody};
use bevy::prelude::*;
use std::f32::consts::*;

use crate::asset_tracking::LoadResource;

use super::movement::ScreenWrap;
use rand::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    /*   app.add_systems(
        FixedUpdate,
        car_velocity
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    ); */
}

pub fn car(car_assets: &CarAssets, init_pos: Vec3, init_vel: Vec3) -> impl Bundle {
    let rng = &mut rand::thread_rng();
    (
        Name::new("Car"),
        Car::default(),
        SceneRoot(car_assets.vehicles.choose(rng).unwrap().clone()),
        ScreenWrap,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        ExternalForce::new(init_vel).with_persistence(true), /* TODO: I have no idea why this needs to be negative */
        /* Answer: because +x is going left */
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(-FRAC_PI_2),
            scale: Vec3::splat(1.8),
        },
    )
}

/* pub fn car_velocity(mut cars: Query<&mut LinearVelocity, With<Car>>) {
    // for (mut velocity) in &mut cars {
    //     velocity = velocity + velocity;
    // }
} */

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CarAssets {
    #[dependency]
    vehicles: Vec<Handle<Scene>>,
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
        }
    }
}
