//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    menus::Menu,
    screens::Screen,
    theme::widget::{self, UiAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            game_title(asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen, &ui_assets),
            widget::button("Settings", open_settings_menu, &ui_assets),
            widget::button("Credits", open_credits_menu, &ui_assets),
            widget::button("Exit", exit_app, &ui_assets),
        ],
        #[cfg(target_family = "wasm")]
        children![
            game_title(asset_server),
            widget::button("Play", enter_loading_or_gameplay_screen, &ui_assets),
            widget::button("Settings", open_settings_menu, &ui_assets),
            widget::button("Credits", open_credits_menu, &ui_assets),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn game_title(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        Name::new("Game Title"),
        Node {
            height: Val::Vh(20.),
            aspect_ratio: Some(2.),
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("images/logotype.png"),

            ..Default::default()
        },
    )
}
