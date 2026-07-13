// support configuring bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// disable console on windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod menus;
mod screens;
mod theme;

use bevy::{asset::AssetMetaCheck, prelude::*};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

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

        // add other plugins.
        app.add_plugins((
            game::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

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

        // set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
        app.configure_sets(FixedUpdate, PausableSystems.run_if(in_state(Pause(false))));

        // spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// Frame update systems
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum FrameSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Fixed update systems
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum FixedSystems {
    /// Apply effects from (de)buffs derive new stats
    ApplyEffects,
    /// Turn, calculate new position, add trail segments
    Movement,
    /// Detect collisions and emit death events
    Collide,
    /// Apply deaths, scoring, round-over transition
    Resolve,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
