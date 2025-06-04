use avian3d::prelude::Collider;
use bevy::{ecs::world, prelude::*, text::cosmic_text::ttf_parser::colr::RadialGradient};

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
    pos_inc_primary: Vec3,
    pos_inc_secondary: Vec3,
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
    let conf: RoadConfig = RoadConfig {
        types: vec![
            LaneType::Border,
            LaneType::LeftToRight,
            LaneType::LeftToRight,
            LaneType::LeftToRight,
            LaneType::LeftToRight,
            LaneType::Separator,
            LaneType::RightToLeft,
            LaneType::RightToLeft,
            LaneType::RightToLeft,
            LaneType::RightToLeft,
            LaneType::Border,
        ],
        pos_start: Vec3::new(-50.0, 0.0, 0.0),
        pos_end: Vec3::new(50.0, 0.0, 0.0),
        pos_inc_primary: Vec3::new(4.0, 0.0, 0.0),
        pos_inc_secondary: Vec3::new(0.0, 0.0, 4.0),
    };

    commands
        .spawn((
            RoadsOrigin,
            Name::new("Roads Origin"),
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            let mut z_offset: f32 =
                -((conf.types.len() / 2) as f32) * (conf.pos_inc_secondary.length() / 2.);

            for lane_type in conf.types.iter() {
                let mut pos: Vec3 = conf.pos_start.with_z(z_offset);

                let road_asset: &Handle<Scene> = match lane_type {
                    LaneType::Border => &road_assets.road_border,
                    LaneType::Separator => &road_assets.road_separator,
                    _ => &road_assets.road_straight,
                };

                while conf.pos_end.with_z(z_offset).distance(pos) >= conf.pos_inc_primary.length() {
                    // info!("Spawning road: {:?} at {}", lane_type, pos);

                    parent.spawn((
                        Road,
                        Name::new("Road"),
                        StateScoped(Screen::Gameplay),
                        Transform::from_translation(pos + Vec3::new(0.0, 0.0, z_offset))
                            .with_scale(Vec3::splat(4.)),
                        SceneRoot(road_asset.clone()),
                        Collider::cuboid(1., 0.4, 1.),
                    ));

                    pos += conf.pos_inc_primary;
                }

                pos += conf.pos_inc_secondary;
                z_offset += conf.pos_inc_secondary.length() / 2.;
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
