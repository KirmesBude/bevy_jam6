use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {}

pub fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("UI Root"),
        StateScoped(Screen::Gameplay),
        children![top_container(), bottom_container(),],
    ));
}

fn bottom_container() -> impl Bundle {
    (
        Name::new("UI Bottom"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(15.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        BackgroundColor(WHITE.into()),
        children![
            widget::button_small("1", |_: Trigger<Pointer<Click>>| {
                print_item(1);
            }),
            widget::button_small("2", |_: Trigger<Pointer<Click>>| {
                print_item(2);
            }),
            widget::button_small("3", |_: Trigger<Pointer<Click>>| {
                print_item(3);
            }),
            widget::button_small("4", |_: Trigger<Pointer<Click>>| {
                print_item(4);
            }),
            widget::button("crash out", |_: Trigger<Pointer<Click>>| {
                /* TODO: Nothing for now */
            }),
        ],
    )
}

fn top_container() -> impl Bundle {
    (
        Name::new("UI Top"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(15.),
            position_type: PositionType::Absolute,
            top: Val::Percent(0.),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        BackgroundColor(BLACK.into()),
        children![widget::label("High Score"), widget::label("Current Combo?"),],
    )
}

fn print_item(index: u8) {
    dbg!("{}", index);
}
