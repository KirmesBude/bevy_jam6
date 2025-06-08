use bevy::{color::palettes::css::*, ecs::spawn::SpawnWith, prelude::*};

use crate::{
    game::{
        pertubator::{ActivePertubator, Pertubator, PertubatorAssets},
        points::HighScore,
    },
    screens::Screen,
    theme::{
        palette::{BUTTON_BACKGROUND, BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND},
        prelude::InteractionPalette,
        widget,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HighScoreUi>();

    app.add_systems(Update, update_highscore);
}

pub fn spawn_game_ui(mut commands: Commands, pertubator_assets: Res<PertubatorAssets>) {
    commands.spawn((
        widget::ui_root("UI Root"),
        StateScoped(Screen::Gameplay),
        children![top_container(), bottom_container(&pertubator_assets),],
    ));
}

fn bottom_container(pertubator_assets: &PertubatorAssets) -> impl Bundle {
    (
        Name::new("UI Bottom"),
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(16.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.), /* TODO: This can be replaced if root ui is SpaceBetween */
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.)),
            ..Default::default()
        },
        BackgroundColor(BLACK.with_alpha(0.33).into()),
        children![item_container(pertubator_assets),],
    )
}

fn item_container(pertubator_assets: &PertubatorAssets) -> impl Bundle {
    (
        Name::new("UI Items"),
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(BLACK.into()),
        children![
            pertubator_button(Pertubator::Spring, pertubator_assets),
            pertubator_button(Pertubator::Nails, pertubator_assets),
            pertubator_button(Pertubator::Soap, pertubator_assets),
        ],
    )
}

fn top_container() -> impl Bundle {
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
            padding: UiRect::all(Val::Percent(1.)),
            ..Default::default()
        },
        BackgroundColor(BLACK.with_alpha(0.33).into()),
        children![
            (Text("Score: ".into()), TextFont::from_font_size(24.0)),
            highscore(),
            // widget::label("Current Combo?"),
            // widget::label("Achievements")
        ],
    )
}

// fn print_item(index: u8) {
//     dbg!("{}", index);
// }

fn pertubator_button(pertubator: Pertubator, pertubator_assets: &PertubatorAssets) -> impl Bundle {
    let image = pertubator_assets.get(&pertubator).unwrap().image().clone();

    (
        Name::new(pertubator.name()),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    Node {
                        padding: UiRect::all(Val::Percent(1.)),
                        ..Default::default()
                    },
                    // BackgroundColor(BUTTON_BACKGROUND.with_alpha(0.33)),
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
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .observe(
                    move |_: Trigger<Pointer<Click>>,
                          mut active_pertubator: ResMut<ActivePertubator>| {
                        active_pertubator.0 = Some(pertubator);
                    },
                );
        })),
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
