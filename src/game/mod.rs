use bevy::app::App;

mod car;
mod car_colliders;
mod car_de_spawning;
mod consts;
mod pertubator;
pub mod points;
mod road;
pub mod ui;
mod util;
mod world;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        road::plugin,
        car::plugin,
        car_colliders::plugin,
        car_de_spawning::plugin,
        world::plugin,
        pertubator::plugin,
        util::plugin,
        points::plugin,
        ui::plugin,
    ));
}

// TODO in this folder: Make everything scoped or despawn manually.
// When leaving the game or resetting the level, cars, obstacles etc. have to be removed
