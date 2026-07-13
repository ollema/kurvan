use bevy::prelude::*;

use crate::{FixedSystems, PausableSystems, Pause};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        steer.in_set(FixedSystems::Movement).in_set(PausableSystems),
    );

    app.add_systems(
        RunFixedMainLoop,
        interpolate_rendered_position
            .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop)
            .run_if(in_state(Pause(false))),
    );
}

/// Steering state and stats.
#[derive(Component)]
pub struct Curve {
    /// Heading in radians, ccw from +x.
    pub angle: f32,
    /// World units per second
    pub speed: f32,
    /// Radians per second. Turning radius is `speed / turn_rate`.
    pub turn_rate: f32,
}

/// Position.
#[derive(Component)]
pub struct PhysicalPosition(pub Vec2);

/// Previous position (used for render interpolation).
#[derive(Component)]
pub struct PreviousPhysicalPosition(pub Vec2);

/// Steering intent.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum TurnInput {
    Left,
    #[default]
    Straight,
    Right,
}

fn steer(
    time: Res<Time>,
    mut query: Query<(
        &TurnInput,
        &mut Curve,
        &mut PhysicalPosition,
        &mut PreviousPhysicalPosition,
    )>,
) {
    let delta_time = time.delta_secs();

    for (turn_input, mut curve, mut position, mut previous_position) in query.iter_mut() {
        // update previous position
        previous_position.0 = position.0;

        // convert turn input into relative direction
        let direction = match turn_input {
            TurnInput::Left => 1.0,
            TurnInput::Straight => 0.0,
            TurnInput::Right => -1.0,
        };

        // update curve angle based on direction, turn rate and advancement of time
        curve.angle += direction * curve.turn_rate * delta_time;

        // update position based on speed, direction and advancement of time
        position.0 += curve.speed * Vec2::from_angle(curve.angle) * delta_time;
    }
}

fn interpolate_rendered_position(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&PhysicalPosition, &PreviousPhysicalPosition, &mut Transform)>,
) {
    let alpha = fixed_time.overstep_fraction();

    for (position, previous_position, mut transform) in query.iter_mut() {
        // interpolate rendered/virtual position based on overstep
        let interpolated_position = previous_position.0.lerp(position.0, alpha);
        // keep current z
        transform.translation = interpolated_position.extend(transform.translation.z);
    }
}
