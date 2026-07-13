//! Resources shared between plugins, one file per resource.
//! Import these via `crate::prelude::*` instead of referencing this module directly.

mod arena;
mod player_mesh;

pub(crate) use arena::*;
pub(crate) use player_mesh::*;
