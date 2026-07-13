//! One plugin per file. Plugins must not import from each other — shared
//! vocabulary lives in `crate::prelude` and `crate::widgets`.

mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod input;
mod interaction;
mod level;
mod main_menu;
mod movement;
mod settings_menu;
mod splash;
mod title;

use bevy::prelude::*;

/// Registers all plugins. The only entry point into this module.
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        #[cfg(feature = "dev")]
        dev_tools::plugin,
        input::plugin,
        interaction::plugin,
        level::plugin,
        main_menu::plugin,
        movement::plugin,
        settings_menu::plugin,
        splash::plugin,
        title::plugin,
    ));
}
