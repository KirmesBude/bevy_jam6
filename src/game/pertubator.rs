use avian3d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*, window::PrimaryWindow};

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, game::car_colliders::WheelCollider,
    screens::Screen,
};

use super::{car::Car, util::Lifetime};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PertubatorAssets>();
    app.init_resource::<ActivePertubator>();

    app.register_type::<PertubatorAssets>();
    app.register_type::<PertubatorAsset>();
    app.register_type::<Pertubator>();
    app.register_type::<ActivePertubator>();

    app.add_systems(
        Update,
        drop_obstacle
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Soaped;

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Nailed;

fn obstacle(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    point: Vec3,
) -> impl Bundle {
    (
        Name::new("Obstacle"),
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
        Transform::from_translation(point).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        RigidBody::Dynamic,
        ExternalForce::default().with_persistence(false),
        Collider::cuboid(1.0, 1.0, 1.0),
    )
}

pub fn drop_obstacle(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (camera, cam_transform) = *camera;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let point = ray.get_point(distance);

    /* TODO: Right, because Left triggers on transition */
    if buttons.pressed(MouseButton::Right) {
        commands.spawn((
            obstacle(
                &mut meshes,
                &mut materials,
                Vec3 {
                    x: point.x,
                    y: 5.,
                    z: point.z,
                },
            ),
            StateScoped(Screen::Gameplay),
        ));
    }
}

/// All pertubator assets
/// Extend SOURCE whenever new kinds of pertubators are added
/// Also needs adjustment if there new assets are necessary
#[derive(Debug, Clone, Resource, Asset, Reflect)]
#[reflect(Resource)]
pub struct PertubatorAssets(HashMap<Pertubator, PertubatorAsset>);

impl PertubatorAssets {
    const SOURCE: [(Pertubator, &'static str); 3] = [
        (Pertubator::Spring, ""),
        (Pertubator::Nails, ""),
        (Pertubator::Soap, ""),
    ];
}

impl FromWorld for PertubatorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self(
            Self::SOURCE
                .iter()
                .map(|(pertubator, scene)| {
                    (
                        *pertubator,
                        PertubatorAsset {
                            scene: assets.load(GltfAssetLabel::Scene(0).from_asset(*scene)),
                        },
                    )
                })
                .collect(),
        )
    }
}

/// Assets corresponding to a specific kind of pertubator
#[derive(Debug, Clone, Reflect)]
pub struct PertubatorAsset {
    scene: Handle<Scene>,
}

/// This defines every Pertubator we have
/// For every addition extend the name and spawn implementations
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub enum Pertubator {
    Spring,
    Nails,
    Soap,
}

impl Pertubator {
    pub fn name(&self) -> &'static str {
        match self {
            Pertubator::Spring => "Spring",
            Pertubator::Nails => "Nails",
            Pertubator::Soap => "Soap",
        }
    }

    fn spawn(
        &self,
        entity_commands: &mut EntityCommands,
        position: Vec3,
        pertubator_assets: &PertubatorAssets,
    ) {
        match self {
            Pertubator::Spring => {
                entity_commands.insert(
                (
                            Name::new(self.name()),
                            *self,
                            SceneRoot(pertubator_assets.0.get(self).unwrap().scene.clone()),
                            Transform::from_translation(position),
                            RigidBody::Kinematic,
                            Collider::cylinder(1.0, 1.0),
                            CollisionEventsEnabled,
                            Lifetime::new(5.),
                        )).observe(|trigger: Trigger<OnCollisionStart>, mut velocity: Query<&mut LinearVelocity>, cars: Query<Entity, With<Car>>,| {
                            let spring = trigger.target(); /* TODO: Extract normal from spring for some shenanigans */
                            let other_entity = trigger.collider;
                            if cars.contains(other_entity) {
                                let mut velocity = velocity.get_mut(spring).unwrap(); /* Unwrap is safe here */
                                dbg!("Car {} triggered spring {}", other_entity, spring);
                                velocity.y = 10.0;
                            }
                        });
            }
            Pertubator::Nails => {
                entity_commands
                    .insert((
                        Name::new(self.name()),
                        *self,
                        SceneRoot(pertubator_assets.0.get(self).unwrap().scene.clone()),
                        Transform::from_translation(position),
                        RigidBody::Static,
                        Collider::cylinder(1.0, 1.0),
                        Sensor,
                        CollisionEventsEnabled,
                        //Lifetime::new(5.),
                    ))
                    .observe(
                        |trigger: Trigger<OnCollisionStart>,
                         mut commands: Commands,
                         wheels: Query<Entity, With<WheelCollider>>| {
                            let nails = trigger.target();
                            let other_entity = trigger.collider;
                            if wheels.contains(other_entity) {
                                commands.entity(other_entity).insert(Nailed);
                                dbg!("Car {} triggered nails {}", other_entity, nails);
                            }
                        },
                    );
            }
            Pertubator::Soap => {
                entity_commands
                    .insert((
                        Name::new(self.name()),
                        *self,
                        SceneRoot(pertubator_assets.0.get(self).unwrap().scene.clone()),
                        Transform::from_translation(position),
                        RigidBody::Static,
                        Collider::cylinder(1.0, 1.0),
                        Sensor,
                        CollisionEventsEnabled,
                        Lifetime::new(5.),
                    ))
                    .observe(
                        |trigger: Trigger<OnCollisionStart>,
                         mut commands: Commands,
                         wheels: Query<Entity, With<WheelCollider>>| {
                            let soap = trigger.target();
                            let other_entity = trigger.collider;
                            if wheels.contains(other_entity) {
                                commands.entity(other_entity).insert(Soaped);
                                dbg!("Car {} triggered soap {}", other_entity, soap);
                            }
                        },
                    );
            }
        }
    }
}

/// This hold the currently active Pertubator as determined by ui selection
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ActivePertubator(pub Option<Pertubator>);

/// Insert this on a picking enabled entity
/// e.g. road
pub fn spawn_pertubator(
    trigger: Trigger<Pointer<Pressed>>,
    mut commands: Commands,
    active_pertubator: Res<ActivePertubator>,
    pertubator_assets: Res<PertubatorAssets>,
) {
    if let Some(pertubator) = active_pertubator.0 {
        if let Some(position) = trigger.hit.position {
            let mut entity_commands = commands.spawn(StateScoped(Screen::Gameplay));
            pertubator.spawn(&mut entity_commands, position, &pertubator_assets);

            dbg!("Spawn {} at {}!", pertubator.name(), position);
        }
    }
}
