use bevy::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        record_steering_input.in_set(FrameSystems::RecordInput),
    );
}

fn record_steering_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Controls, &mut SteeringInput)>,
) {
    for (controls, mut steering_input) in query.iter_mut() {
        let left = input.pressed(controls.left);
        let right = input.pressed(controls.right);
        *steering_input = match (left, right) {
            (true, false) => SteeringInput::Left,
            (false, true) => SteeringInput::Right,
            _ => SteeringInput::Straight,
        }
    }
}
