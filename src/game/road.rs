use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, screens::Screen};

use super::consts::{LANEWIDTH, ROADLENGTH};

#[derive(Debug, Reflect)]
enum LaneType {
    Border,
    LeftToRight,
    Separator,
    RightToLeft,
}

#[derive(Debug, Default, Component, Reflect)]
struct RoadsOrigin;

#[derive(Debug, Default, Component, Reflect)]
struct Road;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<RoadAssets>();
    app.load_resource::<RoadAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_roads);
}

/// Spawn the visuals and the collider of the road.
pub fn spawn_roads(mut commands: Commands, road_assets: Res<RoadAssets>) {
    let lanes = [
        LaneType::Border,
        LaneType::LeftToRight,
        LaneType::LeftToRight,
        LaneType::Border,
        LaneType::Separator,
        LaneType::Border,
        LaneType::RightToLeft,
        LaneType::RightToLeft,
        LaneType::Border,
    ];

    let tiles_per_lane = (ROADLENGTH / LANEWIDTH).round() as u32;

    let total_span_z = lanes.len() as f32 * LANEWIDTH;
    // Start in the neg-neg-quadrant direction. + half lanewidth due to centered meshes.
    let start_x = -ROADLENGTH / 2. + LANEWIDTH / 2.;
    let start_z = -total_span_z / 2. + LANEWIDTH / 2.;

    commands
        .spawn((
            RoadsOrigin,
            Name::new("RoadOrigin"),
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Visibility::default(),
            RigidBody::Static,
            Collider::half_space(Vec3::Y),
            Friction::new(0.05),
        ))
        .with_children(|parent| {
            for (lane_index, lane) in lanes.iter().enumerate() {
                let lane_asset: &Handle<Scene> = match lane {
                    LaneType::Border => &road_assets.road_border,
                    LaneType::Separator => &road_assets.road_separator,
                    _ => &road_assets.road_straight,
                };

                for tile_index in 0..tiles_per_lane {
                    let pos: Vec3 = Vec3::new(
                        start_x + tile_index as f32 * LANEWIDTH,
                        0.,
                        start_z + lane_index as f32 * LANEWIDTH,
                    );

                    parent.spawn((
                        Road,
                        Name::new("Road"),
                        StateScoped(Screen::Gameplay),
                        Transform::from_translation(pos).with_scale(Vec3::splat(4.)),
                        SceneRoot(lane_asset.clone()),
                    ));
                }
            }
        });
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RoadAssets {
    #[dependency]
    pub road_straight: Handle<Scene>,
    #[dependency]
    pub road_border: Handle<Scene>,
    #[dependency]
    pub road_separator: Handle<Scene>,
}

impl FromWorld for RoadAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            road_straight: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/road/road_straight.glb")),
            road_border: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/road/road_border.glb")),
            road_separator: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/road/road_separator.glb")),
        }
    }
}
