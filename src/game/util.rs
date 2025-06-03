use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Lifetime>();

    app.add_systems(
        Update,
        tick_lifetime
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Debug, Component, Reflect)]
pub struct Lifetime(Timer);

impl Lifetime {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Once))
    }
}

fn tick_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        if lifetime.0.tick(time.delta()).just_finished() {
            dbg!("Despawn {}, because lifetime ended", entity);
            commands.entity(entity).despawn();
        }
    }
}
