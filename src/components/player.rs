use bevy::prelude::*;

/// Marker for player entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Player;

/// Keybindings for one player.
#[derive(Component, Clone, Copy)]
pub(crate) struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
}
