use std::ops::{Add, AddAssign};

use bevy::prelude::*;

use crate::game::points::HighScore;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Cash>();
    app.register_type::<Cash>();
}

const POINT_TO_CASH_CONVERSION_RATE: f32 = 0.5;

#[derive(Debug, Default, Resource, Reflect, Clone, Copy)]
#[reflect(Resource)]
pub struct Cash(u32);

impl From<HighScore> for Cash {
    fn from(value: HighScore) -> Self {
        Self((value.get() * POINT_TO_CASH_CONVERSION_RATE).round() as u32)
    }
}

impl Add<Cash> for Cash {
    type Output = Cash;

    fn add(self, rhs: Cash) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Cash {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
