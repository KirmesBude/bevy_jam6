use bevy::prelude::*;
use std::time::Duration;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        car_spawn
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}


#[derive(Component)]
struct CarSpawner;

pub fn car_spawner(position: Vec3, interval: Duration) -> impl Bundle {

    (
        Name::new("car spawner"),
        CarSpawner,
        Timer::new(interval, TimerMode::Repeating),
        Transform::from_translation(position),
    )
}

fn car_spawn(time: Res<Time>, mut car_spawner_query: Query< &CarSpawner>) {
    for mut car_spawner in car_spawner_query {
        car_spawner.timer.tick(time.delta());
    }

    if timer.0.just_finished() {
        println!("Spawn car");
    }


}
