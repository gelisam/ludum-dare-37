use levels::*;
use types::*;


pub struct State {
  pub time: Seconds,
  
  pub message: Option<Message>,
  pub level_number: LevelNumber,
  
  pub player: Player,
  pub spinies: Vec<AnimatedPos>,
}

pub fn initial_state() -> State {
  let t = 0.0;
  let level_number = 0;
  
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
    
    player: Player {
      up_pressed:    false,
      left_pressed:  false,
      down_pressed:  false,
      right_pressed: false,
      most_recent_dir: None,
      buffered_dir:    None,
      pos: AnimatedPos::Idle([0, 1]),
    },
    
    spinies: load_spinies(level_number, t),
  }
}

pub fn load_level(state: &mut State, level_number: LevelNumber, t: Seconds) {
  state.level_number = level_number;
  state.spinies = load_spinies(level_number, t);
}
