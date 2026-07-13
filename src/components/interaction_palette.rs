use bevy::prelude::*;

/// Palette for widget interactions. Add this to a pickable entity, such as a
/// button, to change its [`BackgroundColor`] based on pointer events
/// (hover, press, release).
#[derive(Component, Debug)]
pub(crate) struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}
