use bevy::prelude::*;

use crate::prelude::*;
use crate::widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widgets::button("Play", enter_gameplay_screen),
            widgets::button("Settings", open_settings_menu),
            widgets::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widgets::button("Play", enter_gameplay_screen),
            widgets::button("Settings", open_settings_menu),
        ],
    ));
}

fn enter_gameplay_screen(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
