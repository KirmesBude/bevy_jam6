use avian3d::prelude::*;
use bevy::{prelude::*, render::mesh::CuboidMeshBuilder};

use crate::{
    asset_tracking::LoadResource,
    game::{car_de_spawning::create_car_spawner, pertubator::spawn_pertubator},
    screens::Screen,
};

use super::consts::{GROUNDFRICTION, LANEWIDTH, ROADLENGTH};

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Copy)]

enum LaneType {
    Border,
    LeftToRight,
    Separator,
    RightToLeft,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct RoadsOrigin;

#[derive(Debug, Default, Component, Reflect)]
struct Road;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<RoadAssets>();
    app.load_resource::<RoadAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_roads);
}

/// Spawn the visuals and the collider of the road.
pub fn spawn_roads(
    mut commands: Commands,
    road_assets: Res<RoadAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let lanes = [
        LaneType::Separator,
        LaneType::Border,
        LaneType::LeftToRight,
        LaneType::LeftToRight,
        LaneType::Border,
        LaneType::Separator,
        LaneType::Border,
        LaneType::RightToLeft,
        LaneType::RightToLeft,
        LaneType::Border,
        LaneType::Separator,
    ];

    let tiles_per_lane = (ROADLENGTH / LANEWIDTH).round() as u32;

    let total_span_z = lanes.len() as f32 * LANEWIDTH;
    // Start in the neg-neg-quadrant direction. + half lanewidth due to centered meshes.
    let start_x = -ROADLENGTH / 2. + LANEWIDTH / 2.;
    let start_z = -total_span_z / 2. + LANEWIDTH / 2.;

    let mut car_spawner_info = vec![];

    commands
        .spawn((
            RoadsOrigin,
            Name::new("RoadOrigin"),
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Visibility::default(),
            RigidBody::Static,
            Collider::half_space(Vec3::Y),
            Friction::new(GROUNDFRICTION),
            Mesh3d(
                meshes.add(CuboidMeshBuilder::default().build().scaled_by(Vec3::new(
                    ROADLENGTH,
                    0.1,
                    total_span_z,
                ))), /* I dont like this */
            ),
            Pickable::default(),
        ))
        .observe(spawn_pertubator)
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


                    let mut segment = parent.spawn((
                        Road,
                        Name::new("Road"),
                        StateScoped(Screen::Gameplay),
                        Transform::from_translation(pos).with_scale(Vec3::splat(4.)),
                        SceneRoot(lane_asset.clone()),
                    ));

                    if *lane == LaneType::Separator {
                        segment.insert((RigidBody::Static, Collider::cuboid(1.0, 0.75, 0.8)));
                    }
                }

                if *lane == LaneType::LeftToRight
                    || *lane == LaneType::RightToLeft
                    || *lane == LaneType::Border
                {
                    /* Hackery because a segment does not equal to a lane */
                    let mut current_lane = *lane;
                    if *lane == LaneType::Border {
                        if let Some(next_segment) = lanes.get(lane_index + 1) {
                            if *next_segment == LaneType::LeftToRight || *next_segment == LaneType::RightToLeft {
                                current_lane = *next_segment;
                            }
                        }

                        /* If it did not change it is a "border segment", we dont care about spawning a spawner in */
                        if current_lane == *lane {
                            continue;
                        }
                    }

                    let direction = if current_lane == LaneType::LeftToRight {
                        Vec3::X
                    } else {
                        Vec3::NEG_X
                    };

                    car_spawner_info.push((
                        direction,
                        start_z + lane_index as f32 * LANEWIDTH + LANEWIDTH / 2.0,
                    ));
                }
            }
        });

    for car_spawner_info in car_spawner_info.iter() {
        commands.spawn(create_car_spawner(
            car_spawner_info.1,
            car_spawner_info.0,
            5.,
        ));
    }
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
