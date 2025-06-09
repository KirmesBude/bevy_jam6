//! The screen state for the main gameplay.

use avian3d::prelude::{Physics, PhysicsTime};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{Pause, game::ui::spawn_game_ui, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    /* TODO: Spawn level etc. */
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_game_ui, spawn_tutorial));

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );

    app.register_type::<TutorialTimer>();
    app.init_resource::<TutorialTimer>();
    app.add_systems(
        Update,
        tick_tutorial_timer.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(
    mut next_pause: ResMut<NextState<Pause>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    next_pause.set(Pause(false));

    virtual_time.unpause();

    physics_time.unpause();
}

fn pause(
    mut next_pause: ResMut<NextState<Pause>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    next_pause.set(Pause(true));

    virtual_time.pause();

    physics_time.pause();
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
struct TutorialTimer {
    seen: bool,
    timer: Timer,
}

impl Default for TutorialTimer {
    fn default() -> Self {
        Self {
            seen: false,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn tick_tutorial_timer(time: Res<Time>, mut tutorial_timer: ResMut<TutorialTimer>) {
    tutorial_timer.timer.tick(time.delta());
}

fn spawn_tutorial(
    mut commands: Commands,
    tutorial_timer: Res<TutorialTimer>,
    asset_server: Res<AssetServer>,
) {
    if !tutorial_timer.seen {
        commands
            .spawn((
                StateScoped(Screen::Gameplay),
                Node {
                    position_type: PositionType::Absolute,
                    height: Percent(80.0),
                    width: Percent(80.0),
                    bottom: Percent(12.0),
                    left: Percent(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.8)),
                ImageNode {
                    image: asset_server.load("images/tutorial.png"),
                    ..Default::default()
                },
            ))
            .observe(
                move |trigger: Trigger<Pointer<Click>>,
                      mut commands: Commands,
                      mut tutorial_timer: ResMut<TutorialTimer>| {
                    if tutorial_timer.timer.finished() {
                        commands.entity(trigger.target).despawn();
                        tutorial_timer.seen = true;
                    }
                },
            );
    }
}
