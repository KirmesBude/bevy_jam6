// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod game;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod theme;

use avian3d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    dev_tools::fps_overlay::FpsOverlayPlugin,
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    render::camera::ScalingMode,
};

const ZOOM_SCROLL_FACTOR: f32 = 256.;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Jam6".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        app.add_plugins(FpsOverlayPlugin::default());

        // third party plugins
        app.add_plugins((
            PhysicsPlugins::default(),
            #[cfg(feature = "dev")]
            PhysicsDebugPlugin::default(),
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        // TODO: Move camera and lighting stuff into the game folder.
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, zoom_camera);
        // TODO: Replace by directional light
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 800.,
            ..default()
        });
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 32.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 15.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn zoom_camera(
    projection: Single<&mut Projection>,
    time: Res<Time>,
    acc_scroll: Res<AccumulatedMouseScroll>,
) {
    let scroll_y = match acc_scroll.unit {
        MouseScrollUnit::Pixel => acc_scroll.delta.y / 100.0,
        MouseScrollUnit::Line => acc_scroll.delta.y,
    };

    let delta = scroll_y * ZOOM_SCROLL_FACTOR * time.delta_secs();

    if delta == 0.0 {
        return;
    }

    if let Projection::Orthographic(ref mut ortho) = *projection.into_inner() {
        if let ScalingMode::FixedVertical { viewport_height } = &mut ortho.scaling_mode {
            let autoscale_factor = 1. - (1.0 / (1. + *viewport_height));
            *viewport_height += delta * autoscale_factor;
            *viewport_height = viewport_height.clamp(8., 128.);

            // info!(viewport_height, delta, scroll_y, acc_scroll.delta.y);
        }
    }
}
