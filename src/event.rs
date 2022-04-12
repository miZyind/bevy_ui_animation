use bevy::prelude::Entity;

/// Event raised when an animation completed.
///
/// This event is raised when an tween completed. For non-repeating animations, this is raised once at the end of the animation.
#[derive(Copy, Clone)]
pub struct CompleteEvent {
    /// The [`Entity`] the animation which completed is attached to.
    pub entity: Entity,
}
