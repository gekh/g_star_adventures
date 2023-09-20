use bevy::prelude::*;

// pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
// pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
// pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = {
  let mut style = Style::DEFAULT;

  style.width = Val::Percent(100.0);
  style.height = Val::Percent(100.0);
  style.flex_direction = FlexDirection::Column;
  style.justify_content = JustifyContent::Center;
  style.align_items = AlignItems::Center;
  style.row_gap = Val::Px(8.0);
  style.column_gap = Val::Px(8.0);

  style
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/kenvector_future.ttf"),
    font_size: 22.0,
    color: Color::WHITE,
  }
}

pub const BIG_BUTTON_STYLE: Style = {
  let mut style = Style::DEFAULT;

  style.justify_content = JustifyContent::Center;
  style.align_items = AlignItems::Center;
  style.width = Val::Px(300.0);
  style.height = Val::Px(120.0);

  style
};

pub const BUTTON_STYLE: Style = {
  let mut style = Style::DEFAULT;

  style.justify_content = JustifyContent::Center;
  style.align_items = AlignItems::Center;
  style.width = Val::Px(300.0);
  style.height = Val::Px(80.0);

  style
};

pub const TITLE_STYLE: Style = {
  let mut style = Style::DEFAULT;

  style.flex_direction = FlexDirection::Row;
  style.justify_content = JustifyContent::Center;
  style.align_items = AlignItems::Center;
  style.width = Val::Px(300.0);
  style.height = Val::Px(120.0);

  style
};

pub fn get_big_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/kenvector_future.ttf"),
    font_size: 34.0,
    color: Color::WHITE,
  }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/kenvector_future.ttf"),
    font_size: 28.0,
    color: Color::WHITE,
  }
}
