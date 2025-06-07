use avian3d::prelude::*;
use bevy::{
    math::ops::{exp, sin},
    pbr::NotShadowCaster,
    picking::pointer::PointerInteraction,
    platform::collections::HashMap,
    prelude::*,
    scene::SceneInstanceReady,
    window::PrimaryWindow,
};

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
    app.register_type::<PertubatorPreview>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_preview);
    app.add_systems(
        Update,
        (drop_obstacle, preview_pertubator)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_observer(preview_pertubator_material_transparency);

    app.add_systems(
        FixedUpdate,
        update_springs
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
    const SOURCE: [(Pertubator, (&'static str, &'static str)); 3] = [
        (Pertubator::Spring, ("spring", "images/barrel.png")), /* TODO: Temporary */
        (Pertubator::Nails, ("trap", "images/trap.png")),
        (Pertubator::Soap, ("patch-grass", "images/patch-grass.png")),
    ];
}

impl FromWorld for PertubatorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self(
            Self::SOURCE
                .iter()
                .map(|(pertubator, (scene, image))| {
                    (
                        *pertubator,
                        PertubatorAsset {
                            scene: assets.load(
                                GltfAssetLabel::Scene(0)
                                    .from_asset(format!("models/perturbators/{}.glb", scene)),
                            ),
                            image: assets.load(*image),
                        },
                    )
                })
                .collect(),
        )
    }
}

impl PertubatorAssets {
    pub fn get(&self, pertubator: &Pertubator) -> Option<&PertubatorAsset> {
        self.0.get(pertubator)
    }
}

/// Assets corresponding to a specific kind of pertubator
#[derive(Debug, Clone, Reflect)]
pub struct PertubatorAsset {
    scene: Handle<Scene>,
    image: Handle<Image>,
}

impl PertubatorAsset {
    pub fn scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    pub fn image(&self) -> &Handle<Image> {
        &self.image
    }
}

/// This defines every Pertubator we have
/// For every addition extend the name and spawn implementations
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub enum Pertubator {
    Spring,
    Nails, /* "Trap" */
    Soap,  /* "Sludge" */
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
        let scene = pertubator_assets.get(self).unwrap().scene().clone();
        match self {
            Pertubator::Spring => {
                // Kinematic object for pushing with an activation sensor and a scene as children.
                entity_commands
                    .insert((
                        Name::new(self.name()),
                        Spring { active_time: 0. },
                        *self,
                        Transform::from_translation(position.with_y(spring_y_position(0.))),
                        RigidBody::Kinematic,
                        Collider::cylinder(1.0, 4.0),
                        Visibility::Visible,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Name::new("SpringSensor"),
                                RigidBody::Static,
                                Sensor,
                                Collider::sphere(0.4),
                                CollisionEventsEnabled,
                                Transform::from_xyz(0., 2., 0.),
                            ))
                            .observe(
                                |trigger: Trigger<OnCollisionStart>,
                                 mut commands: Commands,
                                 possible_spring_sensors: Query<&ChildOf, With<Sensor>>,
                                 cars: Query<Entity, With<Car>>| {
                                    let spring_sensor = trigger.target();
                                    let spring =
                                        possible_spring_sensors.get(spring_sensor).unwrap().0;
                                    let other_entity = trigger.collider;
                                    if cars.contains(other_entity) {
                                        commands.entity(spring).insert(Lifetime::new(2.));
                                        commands
                                            .entity(spring_sensor)
                                            .remove::<CollisionEventsEnabled>();
                                    }
                                },
                            );

                        parent.spawn((
                            Name::new("SpringScene"),
                            SceneRoot(scene),
                            Transform::from_xyz(0., 2., 0.),
                        ));
                    });
            }
            Pertubator::Nails => {
                entity_commands
                    .insert((
                        Name::new(self.name()),
                        *self,
                        SceneRoot(scene),
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
                        SceneRoot(scene),
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

#[derive(Clone, Component, Debug, Reflect)]
struct Spring {
    active_time: f32,
}

/// Function for moving springs.
///
/// It starts slightly below zero to avoid contacts with the cars before activating.
fn spring_y_position(active_time: f32) -> f32 {
    // Scale the time to fit well in the function.
    let x = 5. * active_time - 7.3;

    (exp(-x) * sin(2. * x) + 1322.) / 400. - 2. // -2. because of the offset of the collider
}

/// Updates the position of all active springs.
///
/// A spring is activated through adding the Lifetime-component.
fn update_springs(
    springs: Query<(&mut Spring, &Transform, &mut LinearVelocity), With<Lifetime>>,
    time: Res<Time<Physics>>,
) {
    for (mut spring, transform, mut velocity) in springs {
        let time_step = time.delta_secs();

        spring.active_time += time_step;

        let target_position = spring_y_position(spring.active_time);

        velocity.y = (target_position - transform.translation.y) / time_step;
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

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct PertubatorPreview;

fn spawn_preview(mut commands: Commands) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        Name::new("Pertuabtor Preview"),
        PertubatorPreview,
        Transform::default(),
        Visibility::Hidden,
        SceneRoot::default(),
        NotShadowCaster,
    ));
}

/// A system that draws active pertubator preview at hit location
fn preview_pertubator(
    pointers: Query<&PointerInteraction>,
    active_pertubator: Res<ActivePertubator>,
    pertubator_assets: Option<Res<PertubatorAssets>>,
    preview: Single<(&mut Visibility, &mut Transform, &mut SceneRoot), With<PertubatorPreview>>,
) {
    /* Wait on asset load; There is probably a better way */
    let Some(pertubator_assets) = pertubator_assets else {
        return;
    };
    let (mut visiblity, mut transform, mut scene) = preview.into_inner();

    /* Hide the preview in case we do not have a hit or no active pertubator selected*/
    *visiblity = Visibility::Hidden;

    if let Some(pertubator) = active_pertubator.0 {
        /* Update visuals based on pertubator */
        if active_pertubator.is_changed() {
            scene.0 = pertubator_assets.0.get(&pertubator).unwrap().scene.clone();
        }
    } else {
        /* Nothing to do if no active pertubator */
        return;
    }

    /* Update position and visibility */
    for point in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position)
    {
        transform.translation = point;
        *visiblity = Visibility::Inherited;
    }
}

fn preview_pertubator_material_transparency(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    pertubator_preview: Single<Entity, With<PertubatorPreview>>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
) {
    if *pertubator_preview != trigger.target() {
        return;
    }

    // Iterate over all children recursively
    for descendants in children.iter_descendants(trigger.target()) {
        // Get the material of the descendant
        if let Some(material) = mesh_materials
            .get(descendants)
            .ok()
            .and_then(|id| asset_materials.get_mut(id.id()))
        {
            // Create a copy of the material and override alpha
            // Potentially expensive, but probably fine
            let mut new_material = material.clone();
            new_material.alpha_mode = AlphaMode::Blend;
            new_material.base_color.set_alpha(0.66);

            // Override `MeshMaterial3d` with new material
            commands
                .entity(descendants)
                .insert(MeshMaterial3d(asset_materials.add(new_material)));
        }
    }
}
