use bevy::prelude::*;

/// Frame update systems
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum FrameSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else that is frame based.
    Update,
}

/// Fixed update systems
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum FixedSystems {
    /// Apply effects from (de)buffs and derive new stats
    ApplyEffects,
    /// Turn, calculate new position, add trail segments
    Movement,
    /// Detect collisions and emit death events
    Collide,
    /// Apply deaths, scoring, round-over transition
    Resolve,
}
