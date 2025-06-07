use std::f32::consts::{FRAC_PI_2, PI};

/// Width of a lane. Also the sidelength of a road tile.
pub const LANEWIDTH: f32 = 4.;
pub const ROADLENGTH: f32 = 25. * LANEWIDTH;

/// This is from the car origin to the screen border.
pub const DISTANCEUNTILCARSREACHTHEROAD: f32 = 15.;

// Choose a volume that is definitely larger than the largest car.
pub const MAXCARWIDTH: f32 = LANEWIDTH;
pub const MAXCARLENGTH: f32 = 8.;
pub const MAXCARHEIGHT: f32 = 4.;

pub const INITIALCARMODELROTATION: f32 = FRAC_PI_2;

pub const CARFORWARDFORCE: f32 = 15.;

// TODO: Make it different for different cars and obstacles
pub const AIRFRICTIONCOEFFICIENT: f32 = 0.1;
pub const MINIMALVELOCITYFORAIRFRICTION: f32 = 0.1;

pub const WHEELFRICTIONNORMAL: f32 = 0.3;
pub const WHEELFRICTIONSOAPED: f32 = 0.1;
pub const WHEELFRICTIONNAILED: f32 = 0.5;
pub const WHEELFRICTIONSOAPEDANDNAILED: f32 = 0.2;
pub const CARBODYFRICTION: f32 = 0.8;

pub const MAXIMALYAXISANGLEOFFSETFORTORQUECORRECTION: f32 = PI / 180. * 10.; // In rad
pub const MINIMALANGLEOFFSETFORTORQUECORRECTION: f32 = PI / 180. * 1.; // In rad
pub const MAXIMALANGULARVELOCITYFORTORQUECORRECTION: f32 = 2. * PI * 0.1; // In rad per sec
