use bevy::prelude::*;

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum Screen {
    #[default]
    Splash,
    Title,
    Gameplay,
}

/// The game's menu states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum Menu {
    #[default]
    None,
    Main,
    Settings,
}
