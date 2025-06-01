use avian3d::prelude::{Collider, LinearVelocity, Mass, RigidBody};
use bevy::prelude::*;
use std::f32::consts::*;

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

use super::movement::ScreenWrap;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    app.add_systems(
        Update,
        car_velocity
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn car(car_assets: &CarAssets, init_pos: Vec3, init_vel: Vec3) -> impl Bundle {
    (
        Name::new("Car"),
        Car::default(),
        SceneRoot(car_assets.police.clone()),
        ScreenWrap,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity(init_vel), /* TODO: I have no idea why this needs to be negative */
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(-FRAC_PI_2),
            scale: Vec3::splat(1.),
        },
    )
}

/* TODO: This is not right, but friction with the road decreases velocity, which I dont want right now */
pub fn car_velocity(mut cars: Query<&mut LinearVelocity, With<Car>>) {
    for mut velocity in &mut cars {
        *velocity = LinearVelocity(Vec3 {
            x: -4.0,
            y: 0.0,
            z: 0.0,
        });
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CarAssets {
    #[dependency]
    police: Handle<Scene>,
}

impl FromWorld for CarAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            police: assets.load(GltfAssetLabel::Scene(0).from_asset("models/police.glb")),
        }
    }
}
