use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  next_sim_state.set(SimulationState::Paused);
  commands.spawn(AudioBundle {
    source: asset_server.load("audio/GameStarts.ogg"),
    ..default()
  });
}

pub fn resume_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
  next_sim_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
  keyboard_input: Res<Input<KeyCode>>,
  simulation_state: Res<State<SimulationState>>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    match *simulation_state.get() {
      SimulationState::Running => {
        next_sim_state.set(SimulationState::Paused);
        println!("State: Paused");
      }
      SimulationState::Paused => {
        next_sim_state.set(SimulationState::Running);
        println!("State: Running");
      }
    }
  }
}
