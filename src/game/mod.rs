mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod ui;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use crate::AppState;

use self::{systems::*, ui::GameUIPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<SimulationState>()
      // Plugins
      .add_plugins((
        EnemyPlugin,
        PlayerPlugin,
        ScorePlugin,
        StarPlugin,
        GameUIPlugin,
      ))
      // Systems
      .add_systems(OnEnter(AppState::Game), pause_simulation)
      .add_systems(OnExit(AppState::Game), resume_simulation)
      .add_systems(Update, (toggle_simulation).run_if(in_state(AppState::Game)));
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum SimulationState {
  #[default]
  Running,
  Paused,
}
