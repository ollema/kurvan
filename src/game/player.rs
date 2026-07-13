use bevy::prelude::*;

use crate::game::movement::{Curve, PhysicalPosition, PreviousPhysicalPosition, TurnInput};
use crate::{FrameSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerMesh>();

    app.add_systems(
        Update,
        record_turn_input
            .in_set(FrameSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The circle mesh shared by all players, created once at startup.
#[derive(Resource)]
pub struct PlayerMesh(pub Handle<Mesh>);

impl FromWorld for PlayerMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self(meshes.add(Circle::new(1.0)))
    }
}

/// The player character.
pub fn player(
    spawn_pos: Vec2,
    spawn_angle: f32,
    controls: Controls,
    color: Color,
    mesh: Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Transform::from_translation(spawn_pos.extend(1.0)).with_scale(Vec2::splat(5.0).extend(1.0)),
        Curve {
            angle: spawn_angle,
            speed: 100.0,
            turn_rate: 3.5,
        },
        PhysicalPosition(spawn_pos),
        PreviousPhysicalPosition(spawn_pos),
        TurnInput::default(),
        controls,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Player;

/// Keybindings for one player.
#[derive(Component, Clone, Copy)]
pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
}

fn record_turn_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Controls, &mut TurnInput)>,
) {
    for (controls, mut turn_input) in query.iter_mut() {
        let left = input.pressed(controls.left);
        let right = input.pressed(controls.right);
        *turn_input = match (left, right) {
            (true, false) => TurnInput::Left,
            (false, true) => TurnInput::Right,
            _ => TurnInput::Straight,
        }
    }
}
