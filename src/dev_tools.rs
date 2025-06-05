//! Development tools for the game. This plugin is only enabled in dev builds.

use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    color::palettes::tailwind::{PINK_100, RED_500},
    dev_tools::{fps_overlay::FpsOverlayPlugin, states::log_transitions},
    input::common_conditions::input_just_pressed,
    picking::pointer::PointerInteraction,
    prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        );

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.add_systems(Update, draw_mesh_intersections);
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}
