//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    color::palettes::css::WHITE,
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::{
    asset_tracking::LoadResource,
    theme::{interaction::InteractionPalette, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<UiAssets>();
    app.load_resource::<UiAssets>();
}

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>, ui_assets: &UiAssets) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont {
            font: ui_assets.font.clone(),
            font_size: 40.,
            ..Default::default()
        },
        TextColor(HEADER_TEXT),
        TextShadow {
            offset: Vec2::splat(2.5),
            ..Default::default()
        },
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>, ui_assets: &UiAssets) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont {
            font: ui_assets.font.clone(),
            font_size: 24.,
            ..Default::default()
        },
        TextColor(LABEL_TEXT),
        TextShadow {
            offset: Vec2::splat(2.5),
            ..Default::default()
        },
    )
}

pub fn label_simple(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(16.0),
        TextColor(WHITE.into()),
        TextShadow {
            offset: Vec2::splat(2.5),
            ..Default::default()
        },
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I, ui_assets: &UiAssets) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: Px(380.0),
                height: Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
        ui_assets,
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    ui_assets: &UiAssets,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ui_assets,
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
pub fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
    ui_assets: &UiAssets,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    let font = ui_assets.font.clone();
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont {
                            font,
                            font_size: 40.,
                            ..Default::default()
                        },
                        TextColor(BUTTON_TEXT),
                        TextShadow {
                            offset: Vec2::splat(2.5),
                            ..Default::default()
                        },
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

#[derive(Debug, Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct UiAssets {
    #[dependency]
    pub font: Handle<Font>,
    #[dependency]
    pub stop: Handle<Image>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            font: assets.load("fonts/Kenney Future Narrow.ttf"),
            stop: assets.load("images/stop.png"),
        }
    }
}
