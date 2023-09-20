use super::events::PlayerMoved;
use super::{components::*, PLAYER_SIZE, PLAYER_SPEED};
use crate::events::*;
use crate::game::enemy::{components::*, ENEMY_SIZE};
use crate::game::score::resources::Score;
use crate::game::star::{components::Star, STAR_SIZE};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct AnimationIndices {
  first: usize,
  last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
  time: Res<Time>,
  mut query: Query<
    (
      &AnimationIndices,
      &mut AnimationTimer,
      &mut TextureAtlasSprite,
    ),
    With<Player>,
  >,
  mut player_moved_event_reader: EventReader<PlayerMoved>,
) {
  let cnt = player_moved_event_reader.iter().count();

  for (indices, mut timer, mut sprite) in &mut query {
    if cnt == 0 {
      sprite.index = 14;
    } else {
      sprite.index = if sprite.index > indices.last {
        6
      } else {
        sprite.index
      };
      timer.tick(time.delta());
      if timer.just_finished() {
        sprite.index = if sprite.index == indices.last {
          indices.first
        } else {
          sprite.index + 1
        };
      }
    }
  }
}

pub fn spawn_player(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let window = window_query.get_single().unwrap();

  let texture_handle = asset_server.load("sprites/YeOldyKnifeGuy_gimp_color_to_alpha.png");
  let texture_atlas = TextureAtlas::from_grid(
    texture_handle,
    Vec2::new(20.0, 21.0),
    6,
    3,
    None,
    Some(Vec2::new(18.0, 15.0)),
  );
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  // Use only the subset of sprites in the sheet that make up the run animation
  let animation_indices = AnimationIndices { first: 6, last: 11 };

  commands.spawn((
    SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      sprite: TextureAtlasSprite {
        index: animation_indices.first,
        flip_x: true,
        ..default()
      },
      transform: Transform {
        translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
        scale: Vec3::new(3.0, 3.0, 1.0),
        ..default()
      },
      ..default()
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    Player {},
  ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
  if let Ok(player_entity) = player_query.get_single() {
    commands.entity(player_entity).despawn();
  }
}

pub fn player_movement(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
  time: Res<Time>,
  mut player_moved_event_writer: EventWriter<PlayerMoved>,
) {
  if let Ok((mut transform, mut sprite)) = player_query.get_single_mut() {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) {
      direction.x -= 1.0;
      sprite.flip_x = true;
      player_moved_event_writer.send(PlayerMoved);
    }
    if keyboard_input.pressed(KeyCode::Right) {
      direction.x += 1.0;
      sprite.flip_x = false;
      player_moved_event_writer.send(PlayerMoved);
    }
    if keyboard_input.pressed(KeyCode::Up) {
      direction.y += 1.0;
      player_moved_event_writer.send(PlayerMoved);
      // sprite.flip_y = false;
    }
    if keyboard_input.pressed(KeyCode::Down) {
      direction.y -= 1.0;
      player_moved_event_writer.send(PlayerMoved);
      // sprite.flip_y = true;
    }

    if direction.length() > 0.0 {
      direction = direction.normalize();
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
  }
}

pub fn confine_player_movement(
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();
  let half = PLAYER_SIZE / 2.0;
  if let Ok(mut transform) = player_query.get_single_mut() {
    if transform.translation.x < half {
      transform.translation.x = half;
    }
    if transform.translation.x > window.width() - half {
      transform.translation.x = window.width() - half;
    }
    if transform.translation.y < half {
      transform.translation.y = half;
    }
    if transform.translation.y > window.height() - half {
      transform.translation.y = window.height() - half;
    }
  }
}

pub fn enemy_hit_player(
  mut commands: Commands,
  mut game_over_event_writer: EventWriter<GameOver>,
  mut player_query: Query<(Entity, &Transform), With<Player>>,
  enemy_query: Query<&Transform, With<Enemy>>,
  asset_server: Res<AssetServer>,
  score: Res<Score>,
) {
  if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
    for enemy_transform in enemy_query.iter() {
      let distance = player_transform
        .translation
        .distance(enemy_transform.translation);
      let player_radius = PLAYER_SIZE / 2.0;
      let enemy_radius = ENEMY_SIZE / 2.0;
      if distance < player_radius + enemy_radius {
        println!("Game over");
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/explosionCrunch_000.ogg"),
          ..default()
        });
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/GAMEOVER.ogg"),
          ..default()
        });
        commands.entity(player_entity).despawn();
        game_over_event_writer.send(GameOver { score: score.value });
      }
    }
  }
}

pub fn player_hit_star(
  mut commands: Commands,
  mut player_query: Query<&Transform, With<Player>>,
  star_query: Query<(Entity, &Transform), With<Star>>,
  asset_server: Res<AssetServer>,
  mut score: ResMut<Score>,
) {
  if let Ok(player_transform) = player_query.get_single_mut() {
    for (star_entity, star_transform) in star_query.iter() {
      let distance = player_transform
        .translation
        .distance(star_transform.translation);
      let player_radius = PLAYER_SIZE / 2.0;
      let star_radius = STAR_SIZE / 2.0;
      if distance < player_radius + star_radius {
        score.value += 1;
        println!("+1 star");
        commands.spawn(AudioBundle {
          source: asset_server.load("audio/impactMetal_heavy_000.ogg"),
          ..default()
        });
        commands.entity(star_entity).despawn();
      }
    }
  }
}
