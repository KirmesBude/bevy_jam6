use bevy::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct Road;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Road>();

    app.register_type::<RoadAssets>();
    app.load_resource::<RoadAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RoadAssets {
    #[dependency]
    pub road_straight: Handle<SceneRoot>,
}

impl FromWorld for RoadAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            road_straight: assets.load("models/kenney_city-kit-roads/road-straight.glb"),
        }
    }
}
