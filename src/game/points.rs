use bevy::prelude::*;

use crate::game::car::CarCrash;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HighScore>();
    app.init_resource::<HighScore>();

    app.add_systems(Update, update_highscore);
}

const CAR_COLLISION_MULTIPLIER: f32 = 100.;

/// HighScore in f32
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct HighScore(pub f32);

impl HighScore {
    pub fn get(&self) -> f32 {
        self.0
    }
}

fn update_highscore(mut car_crashes: EventReader<CarCrash>, mut high_score: ResMut<HighScore>) {
    for car_crash in car_crashes.read() {
        high_score.0 += car_crash.magnitude * CAR_COLLISION_MULTIPLIER;
    }
}
