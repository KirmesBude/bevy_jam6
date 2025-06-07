use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::car::Car;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HighScore>();
    app.init_resource::<HighScore>();
}

const CAR_COLLISION_MULTIPLIER: f32 = 100.;

/// HighScore in f32
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct HighScore(f32);

/// To be added to a car entity
/// Will observer all car related collisions and update score based on total collision impulse
pub fn car_observer_update_highscore(
    trigger: Trigger<OnCollisionStart>,
    mut high_score: ResMut<HighScore>,
    cars: Query<Entity, With<Car>>,
    collisions: Collisions,
) {
    let car = trigger.target();
    let other_entity = trigger.collider;

    /* Lets say we only care about car on car collision for the points */
    if !cars.contains(other_entity) {
        return;
    }
    dbg!("Colliding");

    if let Some(contact_pair) = collisions.get(car, other_entity) {
        high_score.0 += contact_pair.total_normal_impulse_magnitude() * CAR_COLLISION_MULTIPLIER;
        dbg!("Increase high score to {}", high_score);
    }
}
