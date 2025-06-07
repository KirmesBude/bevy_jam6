use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{
        car::spawn_car,
        consts::{DISTANCEUNTILCARSREACHTHEROAD, ROADLENGTH},
    },
    screens::Screen,
};

use super::{
    car::{Car, CarAssets},
    car_colliders::AllCarColliders,
    consts::{MAXCARHEIGHT, MAXCARLENGTH, MAXCARWIDTH},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CarSpawner>();

    app.add_systems(
        FixedUpdate,
        (update_car_spawners, despawn_cars)
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    // Testing
    app.add_systems(OnEnter(Screen::Gameplay), spawn_test_car_spawner);
}

fn spawn_test_car_spawner(mut commands: Commands) {
    commands.spawn(create_car_spawner(-10., Vec3::X, 4.));
}

/// The car spawner is located `DISTANCEUNTILCARSREACHTHEROAD` units away from the beginning of the road.
///
/// It spawns cars, which accelerate towards the road.
///
/// IT ASSUMES THE LANES ARE ONLY IN +X OR -X DIRECTION!
#[derive(Debug, Default, Component, Reflect)]
pub struct CarSpawner {
    target_velocity: f32,
    driving_direction: Vec3, // This has to be a normalized vector!
}

impl CarSpawner {
    pub fn new(target_velocity: f32, driving_direction: Vec3) -> Self {
        CarSpawner {
            target_velocity,
            driving_direction,
        }
    }
}

/// Returns a `Bundle` representing a car spawner entity.
pub fn create_car_spawner(
    mid_of_lane_coord_z: f32,
    driving_direction: Vec3,
    target_velocity: f32,
) -> impl Bundle {
    assert!(driving_direction == Vec3::X || driving_direction == Vec3::NEG_X);

    (
        Name::new("CarSpawner"),
        Transform::from_xyz(
            -driving_direction.x * (ROADLENGTH / 2. + DISTANCEUNTILCARSREACHTHEROAD),
            MAXCARHEIGHT / 2.,
            mid_of_lane_coord_z,
        ),
        CarSpawner::new(target_velocity, driving_direction),
        StateScoped(Screen::Gameplay),
    )
}

/// Update all car spawners to create cars, if possible.
fn update_car_spawners(
    mut commands: Commands,
    spawners: Query<(&CarSpawner, &Transform)>,
    all_rigid_bodies: Query<&RigidBody>,
    spatial_query: SpatialQuery,
    car_assets: Res<CarAssets>,
    all_car_colliders: Res<AllCarColliders>,
) {
    for (spawner, transform) in spawners.iter() {
        // Check, if there is space to spawn the car
        let spawn_area = ColliderAabb::new(
            transform.translation,
            Vec3::new(MAXCARLENGTH, MAXCARHEIGHT, MAXCARWIDTH) / 2.,
        );

        let colliders_in_spawn = spatial_query.aabb_intersections_with_aabb(spawn_area);

        if colliders_in_spawn
            .iter()
            .filter(|entity| {
                *all_rigid_bodies.get(**entity).unwrap_or(&RigidBody::Static) == RigidBody::Dynamic
            })
            .count()
            > 0
        {
            // If there are any dynamic objects in the spawn area, it is blocked.
            continue;
        }

        let mut entity_commands = commands.spawn_empty();
        spawn_car(
            &mut entity_commands,
            &car_assets,
            &all_car_colliders,
            transform.translation.with_y(0.01),
            spawner.target_velocity,
        );
    }
}

// System for despawning cars that are outside of the visible area.
fn despawn_cars(mut commands: Commands, cars: Query<(Entity, &Transform), With<Car>>) {
    for (entity, transform) in cars.iter() {
        if transform.translation.xz().length() > 2. * ROADLENGTH || transform.translation.y < -10. {
            commands.entity(entity).despawn();
        }
    }
}
