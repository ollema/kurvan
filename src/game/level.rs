use bevy::prelude::*;

use crate::game::player::{Controls, PlayerMesh, player};
use crate::screens::Screen;
use rand::RngExt;
use std::f32::consts::TAU;

/// Logical playfield size, centered on the origin. Independent of window size.
#[derive(Resource)]
pub struct Arena {
    pub size: Vec2,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            size: Vec2::new(1120.0, 630.0),
        }
    }
}

/// No one spawns closer to the arena edge than this.
const SPAWN_MARGIN: f32 = 100.0;

const PLAYER_CONFIGS: [(Color, KeyCode, KeyCode); 2] = [
    (Color::srgb(1.0, 0.2, 0.2), KeyCode::Digit1, KeyCode::KeyQ),
    (
        Color::srgb(0.3, 0.6, 1.0),
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ),
];

/// A system that spawns the level and all players.
pub fn spawn_level(
    mut commands: Commands,
    arena: Res<Arena>,
    player_mesh: Res<PlayerMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let spawn_area = arena.size / 2.0 - Vec2::splat(SPAWN_MARGIN);

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|level| {
            for (color, left, right) in PLAYER_CONFIGS {
                let spawn_pos = Vec2::new(
                    rng.random_range(-spawn_area.x..=spawn_area.x),
                    rng.random_range(-spawn_area.y..=spawn_area.y),
                );
                let spawn_angle = rng.random_range(0.0..TAU);
                level.spawn(player(
                    spawn_pos,
                    spawn_angle,
                    Controls { left, right },
                    color,
                    player_mesh.0.clone(),
                    &mut materials,
                ));
            }
        });
}
