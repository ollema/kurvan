use bevy::prelude::*;

/// Simulation position, updated in `FixedUpdate`. The rendered [`Transform`]
/// is interpolated towards this and must not be written directly.
#[derive(Component)]
pub(crate) struct PhysicalPosition(pub Vec2);

/// Simulation position at the previous fixed tick (used for render interpolation).
#[derive(Component)]
pub(crate) struct PreviousPhysicalPosition(pub Vec2);
