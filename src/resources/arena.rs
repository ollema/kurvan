use bevy::prelude::*;

/// Logical playfield size, centered on the origin. Independent of window size.
#[derive(Resource)]
pub(crate) struct Arena {
    pub size: Vec2,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            size: Vec2::new(1120.0, 630.0),
        }
    }
}
