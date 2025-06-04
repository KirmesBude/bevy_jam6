use avian3d::prelude::*;
use bevy::{color::palettes::css::GREEN, prelude::*, render::mesh::PlaneMeshBuilder};

use crate::screens::Screen;

use super::consts::{GAMEPLANESIZEX, GAMEPLANESIZEY};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_grass, spawn_ground));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_light);
}

fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 600.,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            color: Color::linear_rgb(1., 1., 0.8),
            illuminance: 2_500.,
            shadows_enabled: true,
            ..default()
        },
        StateScoped(Screen::Gameplay),
        Transform::from_xyz(1., 6., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// TODO: Add the missing derives
#[derive(Component)]
pub struct Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create the assets every time the ground is created.
    // They are very small and should not have any perfomance impact
    // If they have, add resources saving the handles.
    let mesh_handle = meshes.add(
        PlaneMeshBuilder::new(Dir3::Y, (GAMEPLANESIZEX, GAMEPLANESIZEY).into())
            .build()
            .translated_by(Vec3::new(0., 0.5, 0.)),
    );

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Ground,
        StateScoped(Screen::Gameplay),
        Transform::from_xyz(0., -0.5, 0.), // have the colliding surface at y=0.
        Mesh3d(mesh_handle),
        MeshMaterial3d(material),
        RigidBody::Static,
        Collider::cuboid(GAMEPLANESIZEX, 1., GAMEPLANESIZEY),
        Friction::new(0.01),
    ));
}

// Only for prototyping
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &texture_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::RENDER_WORLD,
    )
}

const GRASS_SIZE: Vec2 = Vec2::new(1000., 1000.);

fn grass(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Grass"),
        Ground,
        Transform::from_xyz(0., -1., 0.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, GRASS_SIZE).mesh())),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        RigidBody::Static,
        Collider::cuboid(GRASS_SIZE.x, 1., GRASS_SIZE.y),
        Friction::new(0.05),
    )
}

pub fn spawn_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        grass(&mut meshes, &mut materials),
    ));
}
