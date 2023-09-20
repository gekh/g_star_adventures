use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
  build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
  if let Ok(main_menu_entity) = main_menu_query.get_single() {
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
  commands
    .spawn((
      NodeBundle {
        style: MAIN_MENU_STYLE,
        background_color: Color::hex("ffcc00").unwrap().into(),
        ..default()
      },
      MainMenu {},
    ))
    .with_children(|parent| {
      // === Title ===
      parent
        .spawn(ImageBundle {
          style: TITLE_STYLE,
          background_color: Color::NONE.into(),
          // image: asset_server.load("sprites/ui/blue_button00.png").into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "G-Star Adventures",
                get_title_text_style(asset_server),
              )],
              alignment: TextAlignment::Center,
              ..default()
            },
            ..default()
          });
        });

      // === Play Button ===
      parent
        .spawn((
          ButtonBundle {
            style: BIG_BUTTON_STYLE,
            image: asset_server.load("sprites/ui/green_button00.png").into(),
            ..default()
          },
          PlayButton {},
        ))
        .with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Start Game",
                get_big_button_text_style(asset_server),
              )],
              alignment: TextAlignment::Center,
              ..default()
            },
            ..default()
          });
        });

      // === Options Button ===
      parent
        .spawn((
          ButtonBundle {
            style: BUTTON_STYLE,
            image: asset_server.load("sprites/ui/blue_button00.png").into(),
            ..default()
          },
          OptionsButton {},
        ))
        .with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Options",
                get_button_text_style(asset_server),
              )],
              alignment: TextAlignment::Center,
              ..default()
            },
            ..default()
          });
        });

      // === Credits Button ===
      parent
        .spawn((
          ButtonBundle {
            style: BUTTON_STYLE,
            image: asset_server.load("sprites/ui/blue_button00.png").into(),
            ..default()
          },
          QuitButton {},
        ))
        .with_children(|parent| {
          parent.spawn(TextBundle {
            text: Text {
              sections: vec![TextSection::new(
                "Credits",
                get_button_text_style(asset_server),
              )],
              alignment: TextAlignment::Center,
              ..default()
            },
            ..default()
          });
        });

      // === Version ===
      parent.spawn(TextBundle {
        text: Text {
          sections: vec![TextSection::new(
            format!("Version: {}", env!("CARGO_PKG_VERSION")),
            get_title_text_style(asset_server),
          )],
          alignment: TextAlignment::Center,
          ..default()
        },
        ..default()
      });
    })
    .id()
}
