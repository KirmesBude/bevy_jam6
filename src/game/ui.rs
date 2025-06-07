use bevy::{color::palettes::css::GOLD, prelude::*};

use crate::{
    game::{
        pertubator::{ActivePertubator, Pertubator},
        points::HighScore,
    },
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HighScoreUi>();

    app.add_systems(Update, update_highscore);
}

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
            bottom: Val::Percent(0.), /* TODO: This can be replaced if root ui is SpaceBetween */
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.)),
            ..Default::default()
        },
        //BackgroundColor(WHITE.into()),
        children![
            item_container(),
            widget::button("crash out", |_: Trigger<Pointer<Click>>| {
                /* TODO: Nothing for now */
            }),
        ],
    )
}

fn item_container() -> impl Bundle {
    (
        Name::new("UI Items"),
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        //BackgroundColor(RED.into()),
        children![
            pertubator_button(Pertubator::Spring),
            pertubator_button(Pertubator::Nails),
            pertubator_button(Pertubator::Soap),
            widget::button_small("4", |_: Trigger<Pointer<Click>>| {
                print_item(4);
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
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.)),
            ..Default::default()
        },
        //BackgroundColor(BLACK.into()),
        children![
            highscore(),
            widget::label("Current Combo?"),
            widget::label("Achievements")
        ],
    )
}

fn print_item(index: u8) {
    dbg!("{}", index);
}

fn pertubator_button(pertubator: Pertubator) -> impl Bundle {
    widget::button(
        pertubator.name(),
        move |_: Trigger<Pointer<Click>>, mut active_pertubator: ResMut<ActivePertubator>| {
            active_pertubator.0 = Some(pertubator);
        },
    )
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct HighScoreUi;

fn highscore() -> impl Bundle {
    (
        Name::new("High Score"),
        HighScoreUi,
        Text("".into()),
        TextFont::from_font_size(24.0),
        TextColor(GOLD.into()),
    )
}

fn update_highscore(
    highscore: Res<HighScore>,
    mut highscore_ui: Single<&mut Text, With<HighScoreUi>>,
) {
    highscore_ui.0 = format!("{:.0}", highscore.get().round());
}
