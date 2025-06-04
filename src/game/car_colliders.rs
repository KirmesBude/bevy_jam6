use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{prelude::*, render::mesh::MeshAabb};

use crate::screens::Screen;

use super::{car::CarAssets, consts::WHEELFRICTIONNORMAL};

pub fn plugin(app: &mut App) {
    // Important! Use OnEnter Gameplay instead of anything Loading screen related!
    // The loading screen is only used, if the assets are not loaded before.
    app.add_systems(OnEnter(Screen::Gameplay), calculate_car_colliders);
}

/// This component identifies an `Entity` with a `Collider` representing a car wheel.
///
/// Those entities are children of a car root entity with the `Car`-component.
#[derive(Debug, Default, Component, Reflect)]
pub struct WheelCollider;

/// The collection of the colliders of all cars.
///
/// The order is the same as in `CarAssets::vehicles`.
///
/// Cannot implement Reflect because of `Collider`.
#[derive(Debug, Deref, Clone, Resource)]
pub struct AllCarColliders(pub Vec<CarColliders>);

/// A collection of the colliders of a car.
///
/// `l` stands for `left`, `r` for `right`, `f` for `front` and `b` for `back`.
///
/// The `Transform`s are necessary because some basic colliders cannot be
/// shifted and the mesh origin is not in the middle of the simulated object.
/// The body does not need a `Transform` because it is correctly located.
#[derive(Clone, Debug, Default)]
pub struct CarColliders {
    pub body: Collider,
    pub wheel_bl: Collider,
    pub transfrom_bl: Transform,
    pub wheel_br: Collider,
    pub transfrom_br: Transform,
    pub wheel_fl: Collider,
    pub transfrom_fl: Transform,
    pub wheel_fr: Collider,
    pub transfrom_fr: Transform,
}

impl CarColliders {
    pub fn get_wheel_bl_bundle(&self) -> impl Bundle {
        (
            Name::new("wheel-bl-collider"),
            self.wheel_bl.clone(),
            self.transfrom_bl,
            Friction::new(WHEELFRICTIONNORMAL),
            WheelCollider,
        )
    }
    pub fn get_wheel_br_bundle(&self) -> impl Bundle {
        (
            Name::new("wheel-br-collider"),
            self.wheel_br.clone(),
            self.transfrom_br,
            Friction::new(WHEELFRICTIONNORMAL),
            WheelCollider,
        )
    }
    pub fn get_wheel_fl_bundle(&self) -> impl Bundle {
        (
            Name::new("wheel-fl-collider"),
            self.wheel_fl.clone(),
            self.transfrom_fl,
            Friction::new(WHEELFRICTIONNORMAL),
            WheelCollider,
        )
    }
    pub fn get_wheel_fr_bundle(&self) -> impl Bundle {
        (
            Name::new("wheel-fr-collider"),
            self.wheel_fr.clone(),
            self.transfrom_fr,
            Friction::new(WHEELFRICTIONNORMAL),
            WheelCollider,
        )
    }
}

/// System calculating all the car colliders for the loaded car models.
///
/// Use after all `CarAssets` are loaded, it depends on the loaded meshs.
fn calculate_car_colliders(
    mut commands: Commands,
    scenes: Res<Assets<Scene>>,
    meshes: Res<Assets<Mesh>>,
    car_assets: Res<CarAssets>,
) {
    let mut all_colliders = AllCarColliders(Vec::new());

    for car_scene in car_assets.get_scenes() {
        all_colliders.0.push(
            collider_from_car_scene(car_scene, &scenes, &meshes)
                .expect("Could not create all colliders for a car."),
        );
    }

    commands.insert_resource(all_colliders);
}

/// Uses the scene representing a car to calculate the colliders.
fn collider_from_car_scene(
    scene_handle: &Handle<Scene>,
    scenes: &Assets<Scene>,
    meshes: &Assets<Mesh>,
) -> Option<CarColliders> {
    let scene = scenes
        .get(scene_handle)
        .expect("Requested scene does not exist as asset.");
    let mut colliders = CarColliders::default();
    let mut set_parts = [false; 5]; // Set it in the order of the meshes, displayed in the `CarColliders`-class.

    for entity in scene.world.iter_entities() {
        if let Some((name, mesh_handle, parent)) =
            entity.get_components::<(&Name, &Mesh3d, &ChildOf)>()
        {
            let wheel_mesh_transform = scene.world.entity(parent.0).get::<Transform>().unwrap();

            match name.as_str() {
                "body" => {
                    let mesh = meshes
                        .get(mesh_handle)
                        .expect("Mesh from scene does not exist as asset.");
                    colliders.body = Collider::convex_hull_from_mesh(mesh)
                        .expect("Cannot create collider from body mesh.");
                    set_parts[0] = true;
                }
                "wheel-back-left" => {
                    let mesh = meshes
                        .get(mesh_handle)
                        .expect("Mesh from scene does not exist as asset.");
                    let (collider, transform) = get_collider_for_wheel(mesh, wheel_mesh_transform);
                    colliders.wheel_bl = collider;
                    colliders.transfrom_bl = transform;
                    set_parts[1] = true;
                }
                "wheel-back-right" => {
                    let mesh = meshes
                        .get(mesh_handle)
                        .expect("Mesh from scene does not exist as asset.");
                    let (collider, transform) = get_collider_for_wheel(mesh, wheel_mesh_transform);
                    colliders.wheel_br = collider;
                    colliders.transfrom_br = transform;
                    set_parts[2] = true;
                }
                "wheel-front-left" => {
                    let mesh = meshes
                        .get(mesh_handle)
                        .expect("Mesh from scene does not exist as asset.");
                    let (collider, transform) = get_collider_for_wheel(mesh, wheel_mesh_transform);
                    colliders.wheel_fl = collider;
                    colliders.transfrom_fl = transform;
                    set_parts[3] = true;
                }
                "wheel-front-right" => {
                    let mesh = meshes
                        .get(mesh_handle)
                        .expect("Mesh from scene does not exist as asset.");
                    let (collider, transform) = get_collider_for_wheel(mesh, wheel_mesh_transform);
                    colliders.wheel_fr = collider;
                    colliders.transfrom_fr = transform;
                    set_parts[4] = true;
                }
                _ => (),
            }
        }
    }

    if set_parts.iter().all(|b| *b == true) {
        return Some(colliders);
    }

    return None;
}

fn get_collider_for_wheel(mesh: &Mesh, wheel_mesh_transform: &Transform) -> (Collider, Transform) {
    let aabb = mesh.compute_aabb().expect("Cannot compute wheel AABB.");
    let cylinder = Collider::cylinder(aabb.half_extents.y, aabb.half_extents.x);
    let transform = wheel_mesh_transform
        .with_translation(wheel_mesh_transform.translation + Vec3::from(aabb.center))
        .with_rotation(Quat::from_rotation_z(PI / 2.));

    return (cylinder, transform);
}
