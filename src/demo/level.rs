use bevy::prelude::*;

use crate::{
    demo::player::{PlayerMesh, player},
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_mesh: Res<PlayerMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![player(
            400.0,
            player_mesh.0.clone(),
            &mut materials,
            Color::WHITE,
        )],
    ));
}
