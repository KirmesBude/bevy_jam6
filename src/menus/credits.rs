//! The credits menu.

use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    menus::Menu,
    theme::{prelude::*, widget::UiAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<CreditsAssets>();
    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Credits),
        children![
            widget::header("Created by", &ui_assets),
            created_by(),
            widget::header("Assets", &ui_assets),
            assets(),
            widget::button("Back", go_back_on_click, &ui_assets),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["MacTrissy", "The man, the myth, the legend"],
        ["FreakyWaves", "Genius, playboy, philanthropist"],
        ["KirmesBude", "I was here the whole time"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        [
            "Music, SFX and 3d models made or adapted during the jam",
            "CC0 by FreakyWaves",
        ],
        ["Car crash sounds source", "by https://quicksounds.com"],
        ["Font package", "CC0 by Kenney (www.kenney.nl)"],
        ["Toy Car Kit (1.2)", "CC0 by Kenney (www.kenney.nl)"],
        ["Car Kit (2.0)", "CC0 by Kenney (www.kenney.nl)"],
        ["Mini Dungeon (1.5)", "CC0 by Kenney (www.kenney.nl)"],
        ["Survival Kit (2.0)", "CC0 by Kenney (www.kenney.nl)"],
        [
            "Car Engine Loop 96kHz, 4s",
            "CC BY 3.0 by qubodup (opengameart.org)",
        ],
        ["Button SFX", "CC0 by Jaszunio15"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label_simple(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/FreakyWaves - CrashThemAll_credits.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        StateScoped(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}
