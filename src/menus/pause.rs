//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    screens::Screen,
    theme::widget::{self, UiAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Pause),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::header("Game paused", &ui_assets),
            widget::button("Continue", close_menu, &ui_assets),
            widget::button("Settings", open_settings_menu, &ui_assets),
            widget::button("Quit to title", quit_to_title, &ui_assets),
            widget::button("Exit", exit_app, &ui_assets),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::header("Game paused", &ui_assets),
            widget::button("Continue", close_menu, &ui_assets),
            widget::button("Settings", open_settings_menu, &ui_assets),
            widget::button("Quit to title", quit_to_title, &ui_assets),
        ],
    ));
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
