use bevy::{color::palettes::css::*, ecs::spawn::SpawnWith, prelude::*};

use crate::{
    game::{
        pertubator::{ActivePertubator, Pertubator, PertubatorAssets, UnlockedPertubators},
        points_money::HighScore,
    },
    screens::Screen,
    theme::{
        palette::{BUTTON_BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND},
        prelude::InteractionPalette,
        widget::{self, UiAssets},
    },
};

use super::pertubator::Money;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HighScoreUi>();

    app.add_systems(Update, (update_highscore, update_money));
}

pub fn spawn_game_ui(
    mut commands: Commands,
    pertubator_assets: Res<PertubatorAssets>,
    ui_assets: Res<UiAssets>,
    unlocked_pertubators: Res<UnlockedPertubators>,
) {
    commands.spawn((
        widget::ui_root("UI Root"),
        StateScoped(Screen::Gameplay),
        children![
            top_container(&ui_assets),
            bottom_container(&pertubator_assets, &unlocked_pertubators, &ui_assets),
        ],
    ));
}

fn bottom_container(
    pertubator_assets: &PertubatorAssets,
    unlocked_pertubators: &UnlockedPertubators,
    ui_assets: &UiAssets,
) -> impl Bundle {
    (
        Name::new("UI Bottom"),
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(12.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.), /* TODO: This can be replaced if root ui is SpaceBetween */
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        BackgroundColor(BLACK.with_alpha(0.6).into()),
        children![item_container(
            pertubator_assets,
            unlocked_pertubators,
            ui_assets
        ),],
    )
}

fn item_container(
    pertubator_assets: &PertubatorAssets,
    unlocked_pertubators: &UnlockedPertubators,
    ui_assets: &UiAssets,
) -> impl Bundle {
    (
        Name::new("UI Items"),
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(BLACK.with_alpha(0.).into()),
        children![
            pertubator_button(Pertubator::Soap, pertubator_assets, unlocked_pertubators),
            pertubator_button(Pertubator::Nails, pertubator_assets, unlocked_pertubators),
            pertubator_button(Pertubator::Spring, pertubator_assets, unlocked_pertubators),
            pertubator_button(Pertubator::Barrel, pertubator_assets, unlocked_pertubators),
            (
                Node {
                    width: Val::Px(150.),
                    ..default()
                },
                BackgroundColor(BLACK.with_alpha(0.).into()),
            ),
            stop_button(ui_assets)
        ],
    )
}

fn top_container(ui_assets: &UiAssets) -> impl Bundle {
    (
        Name::new("UI Top"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(8.),
            position_type: PositionType::Absolute,
            top: Val::Percent(0.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        BackgroundColor(BLACK.with_alpha(0.6).into()),
        children![
            (
                Text("Score: ".into()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 24.,
                    ..Default::default()
                }
            ),
            highscore(ui_assets),
            (
                Node {
                    width: Val::Px(200.),
                    ..default()
                },
                BackgroundColor(BLACK.with_alpha(0.).into())
            ),
            (
                Text("Money: ".into()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 24.,
                    ..Default::default()
                }
            ),
            money(ui_assets),
            // widget::label("Current Combo?"),
            // widget::label("Achievements")
        ],
    )
}

// fn print_item(index: u8) {
//     dbg!("{}", index);
// }

fn pertubator_button(
    pertubator: Pertubator,
    pertubator_assets: &PertubatorAssets,
    unlocked_pertubators: &UnlockedPertubators,
) -> impl Bundle {
    let image = pertubator_assets.get(&pertubator).unwrap().image().clone();
    let unlocked = unlocked_pertubators.contains(&pertubator);

    (
        Name::new(pertubator.name()),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    Node::default(),
                    BackgroundColor(BUTTON_BACKGROUND.with_alpha(0.6)),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Image"),
                        ImageNode {
                            image,
                            color: if !unlocked {
                                BLACK.with_alpha(0.6).into()
                            } else {
                                Color::default()
                            },
                            ..Default::default()
                        },
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .observe(
                    move |_: Trigger<Pointer<Click>>,
                          mut active_pertubator: ResMut<ActivePertubator>| {
                        if unlocked {
                            active_pertubator.0 = Some(pertubator);
                        }
                    },
                );
        })),
    )
}

fn stop_button(ui_assets: &UiAssets) -> impl Bundle {
    let image = ui_assets.stop.clone();

    (
        Name::new("Stop Button"),
        Node::default(),
        BackgroundColor(BLACK.into()),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    Node { ..default() },
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Image"),
                        ImageNode {
                            image,
                            ..Default::default()
                        },
                        Node { ..default() },
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .observe(
                    move |_: Trigger<Pointer<Click>>,
                          mut next_screen: ResMut<NextState<Screen>>| {
                        next_screen.set(Screen::Shop);
                    },
                );
        })),
    )
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct HighScoreUi;

fn highscore(ui_assets: &UiAssets) -> impl Bundle {
    (
        Name::new("High Score"),
        HighScoreUi,
        Text("".into()),
        TextFont {
            font: ui_assets.font.clone(),
            font_size: 32.,
            ..Default::default()
        },
        TextColor(GOLD.into()),
    )
}

fn update_highscore(
    highscore: Res<HighScore>,
    mut highscore_ui: Single<&mut Text, With<HighScoreUi>>,
) {
    highscore_ui.0 = format!("{:.0}", highscore.get().round());
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct MoneyUi;

fn money(ui_assets: &UiAssets) -> impl Bundle {
    (
        Name::new("Money"),
        MoneyUi,
        Text("".into()),
        TextFont {
            font: ui_assets.font.clone(),
            font_size: 32.,
            ..Default::default()
        },
        TextColor(GOLD.into()),
    )
}

fn update_money(money: Res<Money>, mut highscore_ui: Single<&mut Text, With<MoneyUi>>) {
    highscore_ui.0 = format!("{}", money.0);
}
