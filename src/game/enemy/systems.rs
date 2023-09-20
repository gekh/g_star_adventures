use crate::game::player::components::Player;
use crate::game::player::PLAYER_SIZE;

use super::{components::*, resources::*, ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
  first: usize,
  last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
  time: Res<Time>,
  mut enemy_query: Query<
    (
      &AnimationIndices,
      &mut AnimationTimer,
      &mut TextureAtlasSprite,
    ),
    With<Enemy>,
  >,
) {
  for (indices, mut timer, mut sprite) in enemy_query.iter_mut() {
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

pub fn spawn_enemies(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let window = window_query.get_single().unwrap();
  let player_translation = Vec2::new(window.width() / 2.0, window.height() / 2.0);
  let mut spawned = vec![];

  for _ in 0..NUMBER_OF_ENEMIES {
    let mut counter = 0;
    let mut rand_x = random::<f32>() * window.width();
    let mut rand_y = random::<f32>() * window.height();
    let mut rand_v = Vec2::new(rand_x, rand_y);
    while counter < 100
      && player_translation.distance(rand_v) < PLAYER_SIZE * 3.0
      && spawned
        .iter()
        .any(|&v: &Vec2| v.distance(rand_v) < ENEMY_SIZE + 1.0)
    {
      rand_x = random::<f32>() * window.width();
      rand_y = random::<f32>() * window.height();
      rand_v = Vec2::new(rand_x, rand_y);
      counter += 1;
    }

    spawned.push(rand_v);

    spawn_one_enemy(
      &mut commands,
      &asset_server,
      &mut texture_atlases,
      rand_x,
      rand_y,
    );
  }
}

fn spawn_one_enemy(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
  texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
  rand_x: f32,
  rand_y: f32,
) {
  let texture_handle = asset_server.load("sprites/blob.png");
  let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 28.0), 8, 1, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  let animation_indices = AnimationIndices { first: 0, last: 7 };

  commands.spawn((
    SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      sprite: TextureAtlasSprite {
        index: animation_indices.first,
        flip_x: false,
        ..default()
      },
      transform: Transform {
        translation: Vec3::new(rand_x, rand_y, 1.0),
        scale: Vec3::new(4.0, 4.0, 1.0),
        ..default()
      },
      ..default()
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    Enemy {
      direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
    },
  ));
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
  for enemy_entity in enemy_query.iter() {
    commands.entity(enemy_entity).despawn();
  }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
  for (mut transform, enemy) in enemy_query.iter_mut() {
    let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
    transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
  }
}

pub fn update_enemy_direction(
  mut commands: Commands,
  mut enemy_query: Query<(&Transform, &mut TextureAtlasSprite, &mut Enemy)>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();
  let half = ENEMY_SIZE / 2.0;
  for (transform, mut sprite, mut enemy) in enemy_query.iter_mut() {
    let mut direction_changed = false;
    if transform.translation.x < half || transform.translation.x > window.width() - half {
      enemy.direction.x *= -1.0;
      direction_changed = true;
      sprite.flip_x = if enemy.direction.x > 0.0 { false } else { true };
    }

    if transform.translation.y < half || transform.translation.y > window.height() - half {
      enemy.direction.y *= -1.0;
      direction_changed = true;
    }

    if direction_changed {
      let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
      let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
      let sound_effect = if random::<f32>() > 0.5 {
        sound_effect_1
      } else {
        sound_effect_2
      };

      commands.spawn(AudioBundle {
        source: sound_effect,
        settings: PlaybackSettings {
          volume: bevy::audio::Volume::new_relative(0.1),
          ..default()
        },
        ..default()
      });
    }
  }
}

pub fn confine_enemy_movement(
  mut enemy_query: Query<&mut Transform, With<Enemy>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();
  let half = ENEMY_SIZE / 2.0;

  for mut transform in enemy_query.iter_mut() {
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

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
  enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  player_query: Query<&Transform, With<Player>>,
  enemy_query: Query<&Transform, With<Enemy>>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
  if let Ok(player_transform) = player_query.get_single() {
    if enemy_spawn_timer.timer.finished() {
      let window = window_query.get_single().unwrap();
      let mut rand_x = random::<f32>() * window.width();
      let mut rand_y = random::<f32>() * window.height();
      let mut rand_v = Vec3::new(rand_x, rand_y, 0.0);
      let mut counter = 0;

      while counter < 100
        && player_transform.translation.distance(rand_v) < PLAYER_SIZE * 3.0
        && enemy_query
          .iter()
          .any(|t: &Transform| t.translation.distance(rand_v) < ENEMY_SIZE + 1.0)
      {
        rand_x = random::<f32>() * window.width();
        rand_y = random::<f32>() * window.height();
        rand_v = Vec3::new(rand_x, rand_y, 0.0);
        counter += 1;
      }

      spawn_one_enemy(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        rand_x,
        rand_y,
      );
    }
  }
}

/*
pub fn enemies_collision(mut enemy_query: Query<(&Transform, &mut Enemy)>) {
  let mut combinations = enemy_query.iter_combinations_mut();
  while let Some([(a_transform, mut a_enemy), (b_transform, mut b_enemy)]) =
    combinations.fetch_next()
  {
    if a_transform.translation.distance(b_transform.translation) < ENEMY_SIZE {
      println!(
        "DGB: enemies collsiion: {}",
        a_transform.translation.distance(b_transform.translation)
      );
      a_enemy.direction.x *= -1.0;
      a_enemy.direction.y *= -1.0;
      b_enemy.direction.x *= -1.0;
      b_enemy.direction.y *= -1.0;
    }
  }
}
 */
