//! The main menu (seen on the title screen).

use bevy::{prelude::*, ui::Val::*};

use crate::{
    game::pertubator::{Money, Pertubator, PertubatorAssets, UnlockedPertubators},
    menus::Menu,
    screens::Screen,
    theme::widget::{self, UiAssets, button_base, label},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Shop),
        (spawn_shop_menu, update_unlock_displays).chain(),
    );

    app.add_systems(
        Update,
        update_unlock_displays
            .run_if(in_state(Menu::Shop).and(resource_changed::<UnlockedPertubators>)),
    );

    app.register_type::<UnlockPertubatorDisplay>();
    app.register_type::<UnlockPertubatorDisplayLabel>();
}

fn spawn_shop_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    pertubator_assets: Res<PertubatorAssets>,
    money: Res<Money>,
) {
    commands.spawn((
        widget::ui_root("Utilities Shop Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Shop),
        children![(
            Node {
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::SpaceAround,
                width: Px(700.),
                row_gap: Px(30.0),
                ..default()
            },
            children![
                (
                    widget::header("Utilities Shop", &ui_assets),
                    Node {
                        justify_self: JustifySelf::Center,
                        ..default()
                    }
                ),
                (label(format!("Money: {}", money.0), &ui_assets),),
                unlock_pertubator_widget(&ui_assets, Pertubator::Nails, &pertubator_assets),
                unlock_pertubator_widget(&ui_assets, Pertubator::Spring, &pertubator_assets),
                unlock_pertubator_widget(&ui_assets, Pertubator::Barrel, &pertubator_assets),
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: Px(60.),
                        ..default()
                    },
                    children![
                        back_to_title_button(&ui_assets),
                        next_round_button(&ui_assets),
                    ],
                ),
            ],
        )],
    ));
}

fn start_next_round(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_menu.set(Menu::None);
    next_screen.set(Screen::Gameplay);
}

fn back_to_title(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_menu.set(Menu::None);
    next_screen.set(Screen::Title);
}

fn next_round_button(ui_assets: &UiAssets) -> impl Bundle {
    button_base(
        "Next round",
        start_next_round,
        Node {
            width: Px(310.),
            height: Px(90.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ui_assets,
    )
}

fn back_to_title_button(ui_assets: &UiAssets) -> impl Bundle {
    button_base(
        "Back to Title",
        back_to_title,
        Node {
            width: Px(310.),
            height: Px(90.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ui_assets,
    )
}

fn unlock_pertubator_widget(
    ui_assets: &UiAssets,
    pertubator: Pertubator,
    pertubator_assets: &PertubatorAssets,
) -> impl Bundle {
    let name = pertubator.name();
    let image = pertubator_assets.get(&pertubator).unwrap().image();

    (
        Name::new("Unlock Widget"),
        Node {
            justify_content: JustifyContent::SpaceBetween,
            height: Px(80.),
            ..default()
        },
        children![
            (
                label(name, ui_assets),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ),
            ImageNode {
                image: image.clone(),
                ..default()
            },
            (
                label(format!("Unlock for {}", pertubator.cost()), ui_assets),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                UnlockPertubatorDisplay(pertubator),
            ),
            (
                unlock_button(ui_assets),
                UnlockPertubatorDisplay(pertubator),
            )
        ],
    )
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UnlockPertubatorDisplayLabel(Pertubator);

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UnlockPertubatorDisplay(Pertubator);

fn unlock_button(ui_assets: &UiAssets) -> impl Bundle {
    button_base(
        "UNLOCK",
        unlock_clicked,
        Node {
            width: Px(220.0),
            height: Px(70.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ui_assets,
    )
}

fn unlock_clicked(
    trigger: Trigger<Pointer<Click>>,
    pertubator_displays: Query<&UnlockPertubatorDisplay>,
    mut pertubators: ResMut<UnlockedPertubators>,
    mut money: ResMut<Money>,
    child_of: Query<&ChildOf>,
) {
    if let Ok(child_of) = child_of.get(trigger.target) {
        if let Ok(display) = pertubator_displays.get(child_of.0) {
            let pertubator = display.0;
            if pertubators.contains(&pertubator) {
                // The button should be gone.
                return;
            }
            if money.0 >= pertubator.cost() {
                money.0 -= pertubator.cost();
                pertubators.push(pertubator);
            }
        }
    }
}

fn update_unlock_displays(
    mut commands: Commands,
    mut all_display_labels: Query<(&UnlockPertubatorDisplayLabel, &mut Text)>,
    all_displays: Query<(Entity, &UnlockPertubatorDisplay)>,
    unlockeds: Res<UnlockedPertubators>,
) {
    for (display, mut text) in all_display_labels.iter_mut() {
        if unlockeds.contains(&display.0) {
            text.0 = "Owned!".into();
        }
    }
    for (entity, display) in all_displays.iter() {
        if unlockeds.contains(&display.0) {
            commands.entity(entity).despawn();
        }
    }
}
