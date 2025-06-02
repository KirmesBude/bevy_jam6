use bevy::app::App;

mod camera;
mod car;
mod consts;
pub mod ui;
mod world;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((car::plugin, world::plugin, ui::plugin));
}

// TODO in this folder: Make everything scoped or despawn manually.
// When leaving the game or resetting the level, cars, obstacles etc. have to be removed
