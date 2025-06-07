use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{audio::SpatialScale, prelude::*};
use rand::Rng;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    game::{
        consts::MAXIMALANGULARVELOCITYFORTORQUECORRECTION, points::car_observer_update_highscore,
    },
    screens::Screen,
};

use super::{
    car_colliders::{AllCarColliders, WheelCollider},
    consts::{
        AIRFRICTIONCOEFFICIENT, CARBODYFRICTION, CARFORWARDFORCE, INITIALCARMODELROTATION,
        MAXIMALYAXISANGLEOFFSETFORTORQUECORRECTION, MINIMALANGLEOFFSETFORTORQUECORRECTION,
        MINIMALVELOCITYFORAIRFRICTION, WHEELFRICTIONNAILED, WHEELFRICTIONSOAPED,
        WHEELFRICTIONSOAPEDANDNAILED,
    },
    pertubator::{Nailed, Soaped},
};

#[derive(Debug, Default, Component, Reflect)]
pub struct Car {
    wrecked: bool, // TODO: Mabye make this a tag component.
    target_velocity: f32,
    driving_direction: Vec3, // This has to be a normalized vector!
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    // TODO: Put this in the right schedule
    app.add_systems(
        FixedUpdate,
        (
            air_friction,
            accelerate_cars,
            correct_car_torque,
            update_friction_changes,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
  
    // app.add_systems(
    //     Update,
    //     spawn_test_car
    //         .run_if(in_state(Screen::Gameplay))
    //         .in_set(AppSystems::Update)
    //         .in_set(PausableSystems),
    // );

}

pub fn spawn_car(
    entity_commands: &mut EntityCommands,
    car_assets: &CarAssets,
    all_car_colliders: &AllCarColliders,
    init_pos: Vec3,
    target_velocity: f32,
    driving_direction: Vec3,
) {
    entity_commands
        .insert(create_car(
            car_assets,
            all_car_colliders,
            init_pos,
            target_velocity,
            driving_direction,
        ))
        .insert(CollisionEventsEnabled)
        .observe(car_observer_update_highscore);
}

/// Returns a bundle representing a car.
pub fn create_car(
    car_assets: &CarAssets,
    all_car_colliders: &AllCarColliders,
    init_pos: Vec3,
    target_velocity: f32,
    driving_direction: Vec3,
) -> impl Bundle {
    let rng = &mut rand::thread_rng();

    let car_index = rng.gen_range(0..car_assets.get_scenes().len());
    let scene_handle = car_assets.vehicles[car_index].clone();
    let colliders = &all_car_colliders[car_index];
    /* TODO: This does not work correctly? */
    let rotation = if driving_direction == Vec3::X {
        INITIALCARMODELROTATION
    } else {
        INITIALCARMODELROTATION + PI
    };

    (
        Name::new("Car"),
        Car {
            wrecked: false,
            target_velocity,
            driving_direction,
        },
        StateScoped(Screen::Gameplay),
        // Physics
        Transform {
            translation: init_pos,
            rotation: Quat::from_rotation_y(rotation),
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
        LinearVelocity::default(),
        ExternalForce::default().with_persistence(false),
        ExternalTorque::new(Vec3::ZERO).with_persistence(false),
        Friction::new(CARBODYFRICTION),
        MaxAngularSpeed(4. * 2. * PI),
        // Gfx and audio
        SceneRoot(scene_handle),
        AudioPlayer::new(car_assets.engine_audio.clone()),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_spatial_scale(SpatialScale::new(0.2))
            .with_volume(bevy::audio::Volume::Decibels(-24.))
            .with_speed(rng.gen_range(0.1..0.8)),
    )
}

/// Applies air friction to the moving objects.
///
/// Objects are typically cars, car parts or obstacles.
/// If they are standing still, they are skipped.
fn air_friction(mut moving_objects: Query<(&LinearVelocity, &mut ExternalForce)>) {
    for (velocity, mut applied_force) in moving_objects.iter_mut() {
        // Only apply this friction to high enough velocities to avoid vibrations when standing still
        if velocity.length() < MINIMALVELOCITYFORAIRFRICTION {
            continue;
        }
        // Apply a force in the opposite direction of the velocity.
        // This force is proportional to the square of the velocity with the given factor.
        // It has to be weighted with the time step.
        // The force is cleared by avian every frame.
        let new_force =
            applied_force.force() - AIRFRICTIONCOEFFICIENT * velocity.0.length() * velocity.0;
        applied_force.set_force(new_force);
    }
}

/// Applies the driving force to the cars being not wrecked.
fn accelerate_cars(mut cars: Query<(&Car, &LinearVelocity, &mut ExternalForce)>) {
    for (car, velocity, mut applied_force) in cars.iter_mut() {
        if car.wrecked || velocity.length() > car.target_velocity {
            continue;
        }
        // Let the car accelerate in the trageted direction.
        let new_force = applied_force.force() + car.driving_direction * CARFORWARDFORCE;
        applied_force.set_force(new_force);
    }
}

/// The physics simulation is not perfect and even the symmetric cars are
/// rotating randomly.
///
/// This system tries to minimize the effect of rotation for non-wrecked cars.
fn correct_car_torque(
    mut cars: Query<(
        &Car,
        &AngularVelocity,
        &ComputedAngularInertia,
        &Transform,
        &mut ExternalTorque,
    )>,
) {
    for (car, angular_velocity, inertia, transform, mut torque) in cars.iter_mut() {
        if car.wrecked {
            continue;
        }

        // Do not correct the Y-axis rotation of the car, if the car is too tilted.
        let y_angle_of_transform = transform.rotation.mul_vec3(Vec3::Y).angle_between(Vec3::Y);
        if y_angle_of_transform > MAXIMALYAXISANGLEOFFSETFORTORQUECORRECTION {
            continue;
        }

        // Get the angle with sign between the car rotation and the planned driving direction.
        // The calculated angle is corrensponding to a rotation around - Vec3::Y, because of the use of xz() instead of something like {x}{-z}()
        // This is why we use the minus to bring it into our 3D-space. Afterwards the initial rotation of the entity to fit the model on to the object is subtracted.
        let mut angle_offset = -transform
            .rotation
            .mul_vec3(car.driving_direction)
            .xz()
            .angle_to(car.driving_direction.xz())
            - INITIALCARMODELROTATION;

        if angle_offset < -PI {
            angle_offset += 2. * PI;
        }

        // Do not rotate if the car is in the tolerated range.
        if angle_offset.abs() < MINIMALANGLEOFFSETFORTORQUECORRECTION {
            continue;
        }

        // Do not add additional torque if the car has reached a high angular velocity.
        // This also blocks if the angular velocity is high in the apposite direction.
        // Then "The driver has no influence on the spinning car".
        if angular_velocity.y.abs() > MAXIMALANGULARVELOCITYFORTORQUECORRECTION {
            continue;
        }
        torque.y -= angle_offset * inertia.value().y_axis.y * 0.5;
    }
}

/// This system updates objects affected by a friction changing effect.
///
/// At the moment, only the `Soaped` and the `Nailed` tags exist and are handled.
fn update_friction_changes(
    mut changed_objects: Query<
        (
            &mut Friction,
            Option<&ChildOf>,
            Has<WheelCollider>,
            Has<Soaped>,
            Has<Nailed>,
        ),
        Or<(Added<Soaped>, Added<Nailed>)>,
    >,
    mut cars: Query<&mut Car>,
) {
    for (mut friction, possible_parent, is_wheel, is_soaped, is_nailed) in
        changed_objects.iter_mut()
    {
        let mut set_friction = |val| {
            friction.dynamic_coefficient = val;
            friction.static_coefficient = val;
        };

        // The wheel friction will be applied, if its a wheel or not.
        if is_soaped && is_nailed {
            set_friction(WHEELFRICTIONSOAPEDANDNAILED);
        } else if is_soaped {
            set_friction(WHEELFRICTIONSOAPED);
        } else {
            // Has to be nailed, if Added<Nailed> and not Added<Soaped>
            set_friction(WHEELFRICTIONNAILED);
        }

        // Part of a car -> mark it as wrecked.
        if is_wheel && possible_parent.is_some() {
            let mut parent_car = cars.get_mut(possible_parent.unwrap().0).unwrap();

            // Mark the car as wrecked (-> make the car stop) if anything of this happened.
            // No effect, if the car is already wrecked
            parent_car.wrecked = true;
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
        &self.vehicles
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
                            .from_asset(format!("models/vehicles/{}.glb", model)),
                    )
                })
                .collect(),
            engine_audio: assets.load("audio/sound_effects/engine-loop.ogg"),
        }
    }
}
