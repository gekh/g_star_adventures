use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::components::*;
// use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::AppState;

#[allow(clippy::type_complexity)]
pub fn interact_with_play_button(
  mut commands: Commands,
  mut button_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<PlayButton>)>,
  mut next_app_state: ResMut<NextState<AppState>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((interaction, mut image)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Pressed => {
        *image = asset_server.load("sprites/ui/green_button01.png").into();
        next_app_state.set(AppState::Game);
      }
      Interaction::Hovered => {
        *image = asset_server.load("sprites/ui/green_button01.png").into();
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/Menu-Selection-Click.ogg"),
          ..default()
        });
      }
      Interaction::None => {
        *image = asset_server.load("sprites/ui/green_button00.png").into();
      }
    }
  }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_options_button(
  mut commands: Commands,
  mut button_query: Query<
    (&Interaction, &mut UiImage),
    (Changed<Interaction>, With<OptionsButton>),
  >,
  mut app_exit_event_writer: EventWriter<AppExit>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((interaction, mut image)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Pressed => {
        *image = asset_server.load("sprites/ui/blue_button01.png").into();
        app_exit_event_writer.send(AppExit);
      }
      Interaction::Hovered => {
        *image = asset_server.load("sprites/ui/blue_button01.png").into();
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/Menu-Selection-Click.ogg"),
          ..default()
        });
      }
      Interaction::None => {
        *image = asset_server.load("sprites/ui/blue_button00.png").into();
      }
    }
  }
}

#[allow(clippy::type_complexity)]
pub fn interact_with_quit_button(
  mut commands: Commands,
  mut button_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<QuitButton>)>,
  mut app_exit_event_writer: EventWriter<AppExit>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((interaction, mut image)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Pressed => {
        *image = asset_server.load("sprites/ui/blue_button01.png").into();
        app_exit_event_writer.send(AppExit);
      }
      Interaction::Hovered => {
        *image = asset_server.load("sprites/ui/blue_button01.png").into();
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/Menu-Selection-Click.ogg"),
          ..default()
        });
      }
      Interaction::None => {
        *image = asset_server.load("sprites/ui/blue_button00.png").into();
      }
    }
  }
}
