use opengl_graphics::Texture;

use font::*;


pub const SPRITE_WIDTH:  u32 = 10;
pub const SPRITE_HEIGHT: u32 = 10;

pub struct Resources {
  pub big_font:      Font,
  pub small_font:    Font,
  pub floor:         Texture,
  pub goal_top:      Texture,
  pub goal:          Texture,
  //pub inventory_key: Texture,
  pub key:           Texture,
  pub locked:        Texture,
  pub player:        Texture,
  pub sign:          Texture,
  pub spiny:         Texture,
  pub start_top:     Texture,
  pub start:         Texture,
  pub unlocked_top:  Texture,
  pub unlocked:      Texture,
  pub wall:          Texture,
}

pub fn load_resources() -> Resources {
  use std::path::Path;
  
  Resources {
    big_font:      Font {
                     texture: Texture::from_path(Path::new("images/big-font.png")).unwrap(),
                     zeroth_char: '\x00', grid_width: 25,
                     cell_width:   20.0, cell_height:   20.0,
                     sprite_width: 10.0, sprite_height: 20.0,
                   },
    small_font:    Font {
                     texture: Texture::from_path(Path::new("images/small-font.png")).unwrap(),
                     zeroth_char: '-', grid_width: 13,
                     cell_width:   5.0, cell_height:   5.0,
                     sprite_width: 5.0, sprite_height: 5.0,
                   },
    floor:         Texture::from_path(Path::new("images/floor.png")).unwrap(),
    goal_top:      Texture::from_path(Path::new("images/goal-top.png")).unwrap(),
    goal:          Texture::from_path(Path::new("images/goal.png")).unwrap(),
    //inventory_key: Texture::from_path(Path::new("images/inventory-key.png")).unwrap(),
    key:           Texture::from_path(Path::new("images/key.png")).unwrap(),
    locked:        Texture::from_path(Path::new("images/locked.png")).unwrap(),
    player:        Texture::from_path(Path::new("images/player.png")).unwrap(),
    sign:          Texture::from_path(Path::new("images/sign.png")).unwrap(),
    spiny:         Texture::from_path(Path::new("images/spiny.png")).unwrap(),
    start_top:     Texture::from_path(Path::new("images/start-top.png")).unwrap(),
    start:         Texture::from_path(Path::new("images/start.png")).unwrap(),
    unlocked_top:  Texture::from_path(Path::new("images/unlocked-top.png")).unwrap(),
    unlocked:      Texture::from_path(Path::new("images/unlocked.png")).unwrap(),
    wall:          Texture::from_path(Path::new("images/wall.png")).unwrap(),
  }
}
