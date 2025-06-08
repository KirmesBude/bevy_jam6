//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Shop), open_shop_menu);
    app.add_systems(OnExit(Screen::Shop), close_shop_menu);
}

fn open_shop_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Shop);
}

fn close_shop_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
