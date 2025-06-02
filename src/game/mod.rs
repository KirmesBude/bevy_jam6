use bevy::app::App;

mod camera;
mod car;
mod consts;
mod world;
mod road;
mod pertubator;
mod ui;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        car::plugin,
        world::plugin,
        pertubator::plugin,
    ));
}

// TODO in this folder: Make everything scoped or despawn manually.
// When leaving the game or resetting the level, cars, obstacles etc. have to be removed

