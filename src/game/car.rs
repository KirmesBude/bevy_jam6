use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{audio::SpatialScale, math::ops::acos, prelude::*};
use rand::Rng;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    game::{consts::MAXIMALANGULARVELOCITYFORTORQUECORRECTION, util::Lifetime},
    screens::Screen,
};

use super::{
    car_colliders::{AllCarColliders, WheelCollider},
    consts::{
        CARBODYFRICTION, CARFORWARDFORCE, INITIALCARMODELROTATION,
        MAXIMALYAXISANGLEOFFSETFORTORQUECORRECTION, MINIMALANGLEOFFSETFORTORQUECORRECTION,
        WHEELFRICTIONNAILED, WHEELFRICTIONSOAPED, WHEELFRICTIONSOAPEDANDNAILED,
    },
    pertubator::{Nailed, Soaped},
};

const CRASH_SOUND_MAGNITUDE_CUTOFF_1: f32 = 10.0;
const CRASH_SOUND_MAGNITUDE_CUTOFF_2: f32 = 20.0;
const DEBRIS_IMPULSE: f32 = 50.0;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Wrecked;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Car {
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
        (accelerate_cars, correct_car_torque, update_friction_changes)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        (
            play_crash_sound,
            spawn_debris_on_crash,
            spawn_smoke_on_wrecked,
            remove_audio_on_wrecked,
            rotate_y,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.register_type::<CarCrashable>();
    app.register_type::<CarCrash>();
    app.add_event::<CarCrash>();

    app.register_type::<Wrecked>();
    app.register_type::<RotateY>();
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
        .insert((CollisionEventsEnabled, CarCrashable))
        .observe(car_observer_crash);
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
            .with_volume(bevy::audio::Volume::Decibels(-14.))
            .with_speed(rng.gen_range(0.1..0.8)),
    )
}

/// Applies the driving force to the cars being not wrecked.
fn accelerate_cars(mut cars: Query<(&Car, &LinearVelocity, &mut ExternalForce, Has<Wrecked>)>) {
    for (car, velocity, mut applied_force, has_wrecked) in cars.iter_mut() {
        if has_wrecked || velocity.length() > car.target_velocity {
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
        Has<Wrecked>,
    )>,
) {
    for (car, angular_velocity, inertia, transform, mut torque, has_wrecked) in cars.iter_mut() {
        if has_wrecked {
            continue;
        }

        // Do not correct the Y-axis rotation of the car, if the car is too tilted.
        let y_angle_of_transform = transform.rotation.mul_vec3(Vec3::Y).angle_between(Vec3::Y);
        if y_angle_of_transform > MAXIMALYAXISANGLEOFFSETFORTORQUECORRECTION {
            continue;
        }

        let current_direction = transform
            .rotation
            .mul_quat(Quat::from_rotation_y(-INITIALCARMODELROTATION))
            .mul_vec3(Vec3::X);
        let axis = current_direction.cross(car.driving_direction);

        let angle = acos(current_direction.dot(car.driving_direction));

        // Do not rotate if the car is in the tolerated range.
        if angle.abs() < MINIMALANGLEOFFSETFORTORQUECORRECTION {
            continue;
        }

        // Do not add additional torque if the car has reached a high angular velocity.
        // This also blocks if the angular velocity is high in the apposite direction.
        // Then "The driver has no influence on the spinning car".
        if angular_velocity.y.abs() > MAXIMALANGULARVELOCITYFORTORQUECORRECTION {
            continue;
        }

        torque.y = inertia.tensor().y_axis.y * axis.y.signum();
    }
}

/// This system updates objects affected by a friction changing effect.
///
/// At the moment, only the `Soaped` and the `Nailed` tags exist and are handled.
fn update_friction_changes(
    mut commands: Commands,
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
            if let Some(parent) = possible_parent {
                // Mark the car as wrecked (-> make the car stop) if anything of this happened.
                // No effect, if the car is already wrecked
                commands.entity(parent.0).insert(Wrecked);
            }
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
    #[dependency]
    crash_audio: Vec<Handle<AudioSource>>,
    #[dependency]
    pub explosion_audio: Handle<AudioSource>,
    #[dependency]
    nut: Handle<Scene>,
    #[dependency]
    bolt: Handle<Scene>,
    #[dependency]
    pub smoke: Handle<Scene>,
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
            crash_audio: vec![
                assets.load("audio/sound_effects/crash/small crash.ogg"),
                assets.load("audio/sound_effects/crash/medium_crash.ogg"),
                assets.load("audio/sound_effects/crash/big_crash.ogg"),
            ],
            explosion_audio: assets
                .load("audio/sound_effects/explosion/Grenade Explosion 1 - QuickSounds.com.ogg"),
            nut: assets.load(GltfAssetLabel::Scene(0).from_asset("models/vehicles/debris-nut.glb")),
            bolt: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/vehicles/debris-bolt.glb")),
            smoke: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/vehicles/toy/smoke.glb")),
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct CarCrashable;

// TODO: This can contain other things than cars, so you need to filter for that when reading
#[derive(Debug, Event, Reflect)]
pub struct CarCrash {
    pub entities: [Entity; 2],
    pub magnitude: f32,
}

/// To be added to a car entity
/// Will observer all car related collisions and update score based on total collision impulse
/// TODO: Does this actually trigger twice on a car on car collision?
fn car_observer_crash(
    trigger: Trigger<OnCollisionStart>,
    mut car_crash_writer: EventWriter<CarCrash>,
    car_crashables: Query<Entity, With<CarCrashable>>,
    collisions: Collisions,
) {
    let car = trigger.target();
    let other_entity = trigger.collider;

    /* Allow any car on car crashable collision */
    if car_crashables.contains(other_entity) {
        if let Some(contact_pair) = collisions.get(car, other_entity) {
            let event = CarCrash {
                entities: [car, other_entity],
                magnitude: contact_pair.total_normal_impulse_magnitude(),
            };

            car_crash_writer.write(event);
        }
    }
}

fn play_crash_sound(
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut car_crashes: EventReader<CarCrash>,
    car_assets: Res<CarAssets>,
) {
    for car_crash in car_crashes.read() {
        let transform = transforms.get(car_crash.entities[0]).unwrap();
        let audio_source_index = if car_crash.magnitude < CRASH_SOUND_MAGNITUDE_CUTOFF_1 {
            0
        } else if car_crash.magnitude < CRASH_SOUND_MAGNITUDE_CUTOFF_2 {
            1
        } else {
            2
        };

        commands.spawn((
            Name::new("Crash Sound"),
            StateScoped(Screen::Gameplay),
            *transform,
            Lifetime::new(1.0),
            AudioPlayer::new(car_assets.crash_audio[audio_source_index].clone()),
            PlaybackSettings::ONCE
                .with_spatial(true)
                .with_spatial_scale(SpatialScale::new(0.2))
                .with_volume(bevy::audio::Volume::Linear(0.3)),
        ));
    }
}

// TODO: rng the trajectory
// TODO: Maybe debris should be children so?
fn spawn_debris_on_crash(
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut car_crashes: EventReader<CarCrash>,
    car_assets: Res<CarAssets>,
) {
    for car_crash in car_crashes.read() {
        let transforms = transforms.get_many(car_crash.entities).unwrap();

        for transform in transforms {
            commands.spawn((
                Name::new("Bolt"),
                StateScoped(Screen::Gameplay),
                Transform {
                    translation: transform.translation,
                    scale: Vec3::splat(3.0),
                    ..Default::default()
                },
                Lifetime::new(4.0),
                RigidBody::Dynamic,
                Mass(10.0),
                ExternalImpulse::new(DEBRIS_IMPULSE * Vec3::new(-1., 1., 1.).normalize())
                    .with_persistence(false),
                SceneRoot(car_assets.bolt.clone()),
            ));

            commands.spawn((
                Name::new("Nut"),
                StateScoped(Screen::Gameplay),
                Transform {
                    translation: transform.translation,
                    scale: Vec3::splat(3.0),
                    ..Default::default()
                },
                Lifetime::new(4.0),
                RigidBody::Dynamic,
                Mass(10.0),
                ExternalImpulse::new(DEBRIS_IMPULSE * Vec3::new(1., 1., -1.).normalize())
                    .with_persistence(false),
                SceneRoot(car_assets.nut.clone()),
            ));
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct RotateY;

fn rotate_y(mut transforms: Query<&mut Transform, With<RotateY>>, time: Res<Time>) {
    for mut transform in &mut transforms {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

fn spawn_smoke_on_wrecked(
    mut commands: Commands,
    wrecked_cars: Query<Entity, Added<Wrecked>>,
    car_assets: Res<CarAssets>,
) {
    let smoke = car_assets.smoke.clone();
    for wrecked_car in wrecked_cars {
        let child = commands
            .spawn((
                SceneRoot(smoke.clone()),
                Transform::default()
                    .with_translation(Vec3::new(0., 3., 0.))
                    .with_scale(Vec3::splat(3.0)),
                RotateY,
            ))
            .id();

        commands.entity(wrecked_car).add_child(child);
    }
}

fn remove_audio_on_wrecked(mut commands: Commands, wrecked_cars: Query<Entity, Added<Wrecked>>) {
    for wrecked_car in wrecked_cars {
        commands
            .entity(wrecked_car)
            .remove::<(AudioPlayer, PlaybackSettings, AudioSink)>();
    }
}
