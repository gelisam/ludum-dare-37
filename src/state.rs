use std::collections::VecDeque;

use corpse::*;
use levels::*;
use player::*;
use types::*;


pub struct State {
  pub time: Seconds,
  
  pub message: Option<Message>,
  pub frozen: bool,
  
  pub level_number: LevelNumber,
  pub previous_level: LevelNumber,
  pub next_level: Option<LevelNumber>,
  
  pub player: Player,
  pub corpses: VecDeque<Corpse>,
  
  pub spinies_moving_since: Seconds,
  pub spinies: Vec<MovingSpiny>,
  
  pub temporary_walls: Vec<TemporaryWall>,
}

pub fn initial_state() -> State {
  let t = 0.0;
  let level_number = 1;
  let previous_level = level_number - 1;
  
  State {
    time: t,
    message: Some(".............................................\n\
                   .                                           .\n\
                   .            I've Seen This Room            .\n\
                   .               Twice Already               .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .    made in 72 hours by Samuel Gélineau    .\n\
                   .             for Ludum Dare 37             .\n\
                   .                                           .\n\
                   .              theme: One Room              .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .           press any key to begin          .\n\
                   .                                           .\n\
                   ............................................."),
    frozen: false,
    
    level_number: level_number,
    previous_level: previous_level,
    next_level: None,
    
    player: Player {
      up_pressed:    false,
      left_pressed:  false,
      down_pressed:  false,
      right_pressed: false,
      most_recent_dir: None,
      buffered_dir:    None,
      pos: moving_in(previous_level, level_number, t + PLAYER_MOVE_DURATION),
    },
    corpses: VecDeque::new(),
    
    spinies_moving_since: t,
    spinies: adjust_spinies(Vec::new(), previous_level, level_number),
    
    temporary_walls: adjust_walls(Vec::new(), previous_level, level_number),
  }
}
