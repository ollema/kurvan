use bevy::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, steer.in_set(FixedSystems::Movement));

    app.add_systems(
        RunFixedMainLoop,
        interpolate_rendered_position.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
    );
}

fn steer(
    time: Res<Time>,
    mut query: Query<(
        &SteeringInput,
        &mut SteeringState,
        &mut PhysicalPosition,
        &mut PreviousPhysicalPosition,
    )>,
) {
    let delta_time = time.delta_secs();

    for (steering_input, mut steering, mut position, mut previous_position) in query.iter_mut() {
        // update previous position
        previous_position.0 = position.0;

        // convert steering input into relative direction
        let direction = match steering_input {
            SteeringInput::Left => 1.0,
            SteeringInput::Straight => 0.0,
            SteeringInput::Right => -1.0,
        };

        // update steering angle based on direction, turn rate and advancement of time
        steering.angle += direction * steering.turn_rate * delta_time;

        // update position based on speed, direction and advancement of time
        position.0 += steering.speed * Vec2::from_angle(steering.angle) * delta_time;
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
