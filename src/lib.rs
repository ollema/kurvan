// support configuring bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

mod components;
mod plugins;
mod resources;
mod schedule;
mod states;
mod widgets;

/// Shared vocabulary (components, resources, states, system sets) that plugins
/// may import. Plugins must not import from each other — only from here,
/// `crate::widgets` and `bevy` itself.
pub(crate) mod prelude {
    pub(crate) use crate::components::*;
    pub(crate) use crate::resources::*;
    pub(crate) use crate::schedule::*;
    pub(crate) use crate::states::*;
}

use bevy::{asset::AssetMetaCheck, prelude::*};

use crate::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // add bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // wasm builds will check for meta files (that don't exist) if this isn't set.
                    // this causes errors and even panics on web build on itch.
                    // see https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "akta kurvan!".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // set up global states.
        app.init_state::<Screen>();
        app.init_state::<Menu>();

        // add our plugins.
        app.add_plugins(plugins::plugin);

        // configure frame update systems
        app.configure_sets(
            Update,
            (
                FrameSystems::TickTimers,
                FrameSystems::RecordInput,
                FrameSystems::Update,
            )
                .chain(),
        );

        // configure fixed update systems
        app.configure_sets(
            FixedUpdate,
            (
                FixedSystems::ApplyEffects,
                FixedSystems::Movement,
                FixedSystems::Collide,
                FixedSystems::Resolve,
            )
                .chain(),
        );
    }
}
