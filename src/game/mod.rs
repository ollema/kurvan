use bevy::prelude::*;

use crate::game::level::Arena;

pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Arena>();
    app.add_plugins((movement::plugin, player::plugin));
}
