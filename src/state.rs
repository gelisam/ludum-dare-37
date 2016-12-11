use std::collections::VecDeque;

use corpse::*;
use levels::*;
use player::*;
use types::*;


pub struct State {
  pub time: Seconds,
  
  pub message: Option<Message>,
  pub level_number: LevelNumber,
  pub previous_level: LevelNumber,
  
  pub player: Player,
  pub corpses: VecDeque<Corpse>,
  
  pub spinies_moving_since: Seconds,
  pub spinies: Vec<MovingPos>,
}

pub fn initial_state() -> State {
  let t = 0.0;
  let previous_level = 0;
  let level_number = 1;
  
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
    level_number: level_number,
    previous_level: previous_level,
    
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
    spinies: load_spinies(level_number),
  }
}

pub fn load_level(state: &mut State, level_number: LevelNumber, t: Seconds) {
  state.level_number = level_number;
  
  state.spinies_moving_since = t;
  state.spinies = load_spinies(level_number);
}
