use bevy::{prelude::*, text::cosmic_text::ttf_parser::colr::RadialGradient};

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource, screens::Screen};

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
pub struct RoadConfig {
    types: Vec<LaneType>,
    pos_start: Vec3,
    pos_end: Vec3,
    pos_inc: Vec3,
}

#[derive(Debug, Default, Component, Reflect)]

struct Road;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<RoadConfig>();

    app.register_type::<RoadAssets>();
    app.load_resource::<RoadAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), (spawn_roads));
}

pub fn spawn_roads(mut commands: Commands, road_assets: Res<RoadAssets>) {
    let road_config: RoadConfig = RoadConfig {
        types: vec![
            LaneType::Border,
            LaneType::LeftToRight,
            LaneType::LeftToRight,
            LaneType::Separator,
            LaneType::RightToLeft,
            LaneType::RightToLeft,
            LaneType::Border,
        ],
        pos_start: Vec3::new(-64.0, 0.0, 0.0),
        pos_end: Vec3::new(64.0, 0.0, 0.0),
        pos_inc: Vec3::new(1.0, 0.0, 0.0),
    };

    commands
        .spawn((
            RoadsOrigin,
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for lane_type in road_config.types.iter() {
                let mut pos: Vec3 = road_config.pos_start;

                while road_config.pos_end.distance(pos) > road_config.pos_inc.length() {
                    info!("Spawning road: {:?} at {}", lane_type, pos);

                    parent.spawn((
                        Road,
                        StateScoped(Screen::Gameplay),
                        Transform::from_translation(pos),
                        SceneRoot(road_assets.road_straight.clone()),
                    ));

                    pos += road_config.pos_inc;
                }
            }
        });
}

pub fn single_lane() {}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RoadAssets {
    #[dependency]
    pub road_straight: Handle<Scene>,
}

impl FromWorld for RoadAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            road_straight: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/road/road_straight.glb")),
        }
    }
}
