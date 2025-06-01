use bevy::prelude::*;
use std::time::Duration;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        car_spawn_update
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}


#[derive(Component)]
struct CarSpawner {
    timer: Timer
}
impl CarSpawner {
    fn new(interval: Duration) -> CarSpawner {
        CarSpawner {
            timer: Timer::new(interval, TimerMode::Repeating)
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

fn car_spawn_update(time: Res<Time>, car_spawner_query: Query<&mut CarSpawner>) {
    for mut car_spawner in car_spawner_query {
        car_spawner.timer.tick(time.delta());

        if car_spawner.timer.just_finished() {
            println!("Spawn car");
        }
    }
}
