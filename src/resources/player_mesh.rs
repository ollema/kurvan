use bevy::prelude::*;

/// The circle mesh shared by all players, created once at startup.
#[derive(Resource)]
pub(crate) struct PlayerMesh(pub Handle<Mesh>);

impl FromWorld for PlayerMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self(meshes.add(Circle::new(1.0)))
    }
}
