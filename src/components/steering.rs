use bevy::prelude::*;

/// Steering state and stats.
#[derive(Component)]
pub(crate) struct SteeringState {
    /// Heading in radians, ccw from +x.
    pub angle: f32,
    /// World units per second
    pub speed: f32,
    /// Radians per second. Turning radius is `speed / turn_rate`.
    pub turn_rate: f32,
}

/// Steering intent.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SteeringInput {
    Left,
    #[default]
    Straight,
    Right,
}
