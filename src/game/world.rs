use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};
use avian3d::prelude::*;

use crate::screens::Screen;

use super::consts::{GAMEPLANESIZEX, GAMEPLANESIZEY};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ground);
}


// TODO: Add the missing derives
#[derive(Component)]
pub struct Ground;


pub fn spawn_ground(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Create the assets every time the ground is created.
    // They are very small and should not have any perfomance impact
    // If they have, add resources saving the handles.
    let mesh_handle = meshes.add(PlaneMeshBuilder::new(Dir3::Y, (GAMEPLANESIZEX, GAMEPLANESIZEY).into()).build());

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });


    commands.spawn((
        Ground,
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
