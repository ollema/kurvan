// support configuring bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// disable console on windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use kurvan::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
