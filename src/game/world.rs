use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{GREEN, ORANGE_RED},
    ecs::spawn::SpawnWith,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
};
use rand::Rng;

use crate::{
    asset_tracking::LoadResource,
    game::consts::{LANEWIDTH, ROADLENGTH},
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.load_resource::<WorldAssets>();
    app.register_type::<WorldAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_grass);
    app.add_systems(OnEnter(Screen::Gameplay), spawn_light);

    app.register_type::<MusicAssets>();
    app.load_resource::<MusicAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), start_game_music);
    // app.add_systems(Update, swap_game_music);
}

fn spawn_light(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        StateScoped(Screen::Gameplay),
        DirectionalLight {
            illuminance: 1.0 * light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI / 2., -PI / 4.)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 200.0,
            ..default()
        }
        .build(),
    ));
}

#[derive(Debug, Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldAssets {
    grass: Handle<Scene>,
    grass_large: Handle<Scene>,
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            grass: assets.load(GltfAssetLabel::Scene(0).from_asset("models/misc/grass.glb")),
            grass_large: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/misc/grass-large.glb")),
        }
    }
}

// TODO: Add the missing derives
#[derive(Component)]
pub struct Ground;

const GRASS_SIZE: Vec2 = Vec2::new(150., 100.);

fn grass(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    world_assets: &WorldAssets,
) -> impl Bundle {
    let grass = world_assets.grass.clone();
    let grass_large = world_assets.grass_large.clone();

    (
        Name::new("Ground"),
        Ground,
        Transform::from_xyz(0., -0.01, 0.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, GRASS_SIZE).mesh())),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let rng = &mut rand::thread_rng();

            let x = -ROADLENGTH / 2.0;
            let z = 6. * LANEWIDTH; // Harcoded based on the lane we currently have

            let amount = (ROADLENGTH / LANEWIDTH) as i32;
            for i in 0..amount {
                for j in -amount..amount {
                    if j == 0 {
                        continue;
                    }
                    let x_rand = rng.gen_range((-LANEWIDTH / 2.0)..(LANEWIDTH / 2.0));
                    let z_rand = rng.gen_range((-LANEWIDTH / 2.0)..(LANEWIDTH / 2.0));
                    let z_rand_2 = rng.gen_range((-LANEWIDTH / 2.0)..(LANEWIDTH / 2.0));

                    let grass_rand = rng.gen_range(0..=1);
                    let scene = if grass_rand == 0 {
                        grass.clone()
                    } else {
                        grass_large.clone()
                    };

                    parent.spawn((
                        Name::new("Grass"),
                        Transform::from_xyz(
                            x + i as f32 * LANEWIDTH + x_rand,
                            0.,
                            z * j.signum() as f32 + j as f32 * LANEWIDTH + z_rand + z_rand_2,
                        )
                        .with_scale(3. * Vec3::ONE),
                        SceneRoot(scene),
                    ));
                }
            }
        })),
    )
}

pub fn spawn_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_assets: Res<WorldAssets>,
) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        grass(&mut meshes, &mut materials, &world_assets),
    ));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct MusicAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/FreakyWaves - CrashThemAll.ogg"),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameMusic;

fn start_game_music(mut commands: Commands, game_music: Res<MusicAssets>) {
    commands.spawn((
        Name::new("Game Music"),
        StateScoped(Screen::Gameplay),
        AudioPlayer(game_music.music.clone()),
        PlaybackSettings::LOOP,
        GameMusic,
    ));
}

// fn swap_game_music(mut commands: Commands, game_music: Res<MusicAssets>) {}
