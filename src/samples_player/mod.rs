//! Contains all the types of sample players,
//! use SamplesPlayer for speed and ExactSamplesPlayer for control

pub mod modifiers;

mod samples_player_trait;
pub use samples_player_trait::SamplesPlayerTrait;

mod samples_player;
pub use samples_player::SamplesPlayer;
mod exact_samples_player;
pub use exact_samples_player::ExactSamplesPlayer;