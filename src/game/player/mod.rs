use bevy::prelude::*;

use crate::AppState;

use self::events::*;
use self::systems::*;
use super::SimulationState;

pub mod components;
pub mod events;
pub mod systems;

pub const PLAYER_SIZE: f32 = 15.0;
pub const PLAYER_SPEED: f32 = 200.0;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct MovementSystemSet;
#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<PlayerMoved>()
      .configure_set(Update, MovementSystemSet.before(ConfinementSystemSet))
      .add_systems(OnEnter(AppState::Game), spawn_player)
      .add_systems(OnExit(AppState::Game), despawn_player)
      .add_systems(
        Update,
        (
          animate_sprite.after(confine_player_movement),
          player_movement.in_set(MovementSystemSet),
          confine_player_movement.in_set(ConfinementSystemSet),
          enemy_hit_player,
          player_hit_star,
        )
          .run_if(in_state(AppState::Game))
          .run_if(in_state(SimulationState::Running)),
      );
  }
}
