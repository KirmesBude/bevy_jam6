use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use super::movement::ScreenWrap;

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();
}

pub fn car(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Car"),
        Car::default(),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        ScreenWrap,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity(Vec3 {
            x: -4.0,
            y: 0.0,
            z: 0.0,
        }), /* TODO: I have no idea why this needs to be negative */
    )
}
