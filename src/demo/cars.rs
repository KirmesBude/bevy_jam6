use bevy::{ecs::system::command, prelude::*};
use std::time::Duration;

use avian3d::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_systems(
        FixedUpdate,
        ((car_spawn, car_move, car_despawn)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),),
    );
}

#[derive(Component, Debug)]
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
        Name::new("car spawner"),
        CarSpawner::new(interval),
        Transform::from_translation(position),
    )
}

fn car(init_pos: Vec3, speed: f32, value: u64) -> impl Bundle {
    (
        Car { speed, value },
        RigidBody::Kinematic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_translation(init_pos),
    )
}

fn car_spawn(time: Res<Time>, mut command: Commands, car_spawner_query: Query<&mut CarSpawner>) {
    for mut car_spawner in car_spawner_query {
        car_spawner.timer.tick(time.delta());

        if car_spawner.timer.just_finished() {
            println!("Spawn car");
            command.spawn((
                Car {
                    speed: 1.0,
                    value: 100,
                },
                RigidBody::Kinematic,
                Collider::cuboid(1.0, 1.0, 1.0),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ));
        }
    }
}

fn car_move(time: Res<Time>, car_query: Query<(Entity, &mut Transform, &Car)>) {
    for (entity, mut transform, car) in car_query {
        transform.translation.x += car.speed * time.delta_secs();
        dbg!(entity, transform.translation.x, car);
    }
}


fn car_despawn(car_query: Query<(Entity, &mut Transform, &Car)>){
    for (entity, mut transform, car) in car_query {
        if transform.translation.x > 10.0 {
            command::entity(ent).despawn();
        }
    }
}
