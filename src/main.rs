pub mod events;
mod game;
mod main_menu;
mod systems;

use events::GameOver;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
  #[default]
  MainMenu,
  Game,
  GameOver,
}

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "G-Star Adventures".to_string(),
            ..Default::default()
          }),
          ..Default::default()
        })
        .set(
          ImagePlugin::default_nearest(), // pixel perfect
        ),
    )
    .add_state::<AppState>()
    .add_event::<GameOver>()
    // Plugins
    .add_plugins((GamePlugin, MainMenuPlugin))
    // Systems
    .add_systems(Startup, spawn_camera)
    .add_systems(
      Update,
      (
        transition_to_game_state,
        transition_to_main_menu_state,
        exit_game,
        handle_game_over,
      ),
    )
    .run();
}
