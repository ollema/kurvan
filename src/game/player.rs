use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::movement::{MovementController, ScreenWrap},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerMesh>();

    // record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
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
    max_speed: f32,
    mesh: Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    color: Color,
) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_scale(Vec2::splat(5.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // this should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
