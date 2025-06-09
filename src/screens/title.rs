//! The title screen that appears after the splash screen.

use bevy::{color::palettes::css::DARK_KHAKI, prelude::*};
use rand::Rng;

use crate::{game::car::CarAssets, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Rotating>();
    app.add_systems(
        OnEnter(Screen::Title),
        (update_clear_color, spawn_rotating_cars),
    );
    app.add_systems(Update, rotate.run_if(in_state(Screen::Title)));

    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Rotating;

fn spawn_rotating_cars(mut commands: Commands, car_assets: Res<CarAssets>) {
    let rng = &mut rand::thread_rng();
    const DISTANCE: f32 = 15.;
    const SCALE: f32 = 3.;

    for i in -4..=4 {
        for j in -4..=4 {
            let car_index = rng.gen_range(0..car_assets.get_scenes().len());
            let scene = car_assets.get_scenes()[car_index].clone();
            let translation = Vec3::new(i as f32 * DISTANCE, 0.0, j as f32 * DISTANCE);

            commands.spawn((
                Name::new("Title Car"),
                SceneRoot(scene),
                Transform::from_translation(translation).with_scale(SCALE * Vec3::ONE),
                Rotating,
                StateScoped(Screen::Title),
            ));
        }
    }
}

fn rotate(mut transforms: Query<&mut Transform, With<Rotating>>, time: Res<Time>) {
    for mut transform in &mut transforms {
        transform.rotate_x(time.delta_secs() / 2.);
        transform.rotate_y(time.delta_secs() / 2.);
        transform.rotate_z(time.delta_secs() / 2.);
    }
}

fn update_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = DARK_KHAKI.into();
}
