use bevy::prelude::*;
use std::time::Duration;

use avian3d::prelude::*;

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Car>();

    app.register_type::<CarAssets>();
    app.load_resource::<CarAssets>();

    app.add_plugins((PhysicsPlugins::default(),));
    app.add_systems(
        FixedUpdate,
        ((car_spawn)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),),
    );
    app.add_systems(Update, (car_move, car_despawn).in_set(PausableSystems));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct Car {
    speed: f32,
    value: u64,
}

#[derive(Component)]
struct CarSpawner {
    timer: Timer,
}

impl CarSpawner {
    fn new(interval: Duration) -> CarSpawner {
        CarSpawner {
            timer: Timer::new(interval, TimerMode::Repeating),
        }
    }
}

pub fn car_spawner(position: Vec3, interval: Duration) -> impl Bundle {
    (
        Name::new("CarSpawner"),
        CarSpawner::new(interval),
        Transform::from_translation(position),
    )
}

fn car(init_pos: Vec3, speed: f32, value: u64, car_assets: &CarAssets) -> impl Bundle {
    (
        Name::new("Car"),
        Car { speed, value },
        RigidBody::Kinematic,
        Mesh3d(car_assets.mesh.clone()),
        ShowAabbGizmo {
            color: Some(Color::srgb(1.0, 0., 0.)),
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_translation(init_pos),
    )
}

fn car_spawn(time: Res<Time>, mut commands: Commands, car_spawner_query: Query<&mut CarSpawner>) {
    for mut car_spawner in car_spawner_query {
        car_spawner.timer.tick(time.delta());

        if car_spawner.timer.just_finished() {
            println!("Spawn car");
            commands.spawn(car(Vec3::new(0.0, 0.0, 0.0), 1.0, 10, car_assets));
        }
    }
}

fn car_move(time: Res<Time>, car_query: Query<(Entity, &mut Transform, &Car)>) {
    for (entity, mut transform, car) in car_query {
        transform.translation.x += car.speed * time.delta_secs();
        dbg!(entity, transform.translation.x, car);
    }
}

fn car_despawn(mut commands: Commands, car_query: Query<(Entity, &Transform, &Car)>) {
    for (ent, transform, car) in car_query {
        if transform.translation.x > 10.0 {
            commands.entity(ent).despawn();
            print!("Despawn car {} {}", car.value, ent);
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CarAssets {
    #[dependency]
    mesh: Handle<Mesh>,
}

impl FromWorld for CarAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            mesh: assets.load("models/vehicle_racer.glb"),
        }
    }
}
