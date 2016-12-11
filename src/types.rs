use graphics::math::Vec2d;


pub type Dir = Vec2d<i8>;
pub const UP:    Dir = [ 0,-1];
pub const LEFT:  Dir = [-1, 0];
pub const DOWN:  Dir = [ 0, 1];
pub const RIGHT: Dir = [ 1, 0];

pub type Message = &'static str;
pub type LevelNumber = u8;
pub type Lifetime = u8;
pub type Pos = Vec2d<i8>;
pub type Seconds = f64;
pub type Radians = f64;

pub enum RawInputEvent {
  TimePasses(Seconds),
  PressUp,    ReleaseUp,
  PressLeft,  ReleaseLeft,
  PressDown,  ReleaseDown,
  PressRight, ReleaseRight,
  PressPause,
  PressAnyKey,
}

pub enum Action {
  Move(Pos, Dir),
  ReadSign(Message),
  Pause, Unpause,
}


pub enum AnimatedPos {
  Idle(Pos),
  MovingSince(Pos, Dir, Seconds),
}

pub struct State {
  pub time: Seconds,
  
  pub message: Option<Message>,
  pub level_number: LevelNumber,
  
  pub up_pressed:    bool,
  pub left_pressed:  bool,
  pub down_pressed:  bool,
  pub right_pressed: bool,
  pub most_recent_dir: Option<Dir>, // favour the last key if many are pressed
  pub buffered_dir:    Option<Dir>, // a key tap which hasn't been honored yet
  pub player: AnimatedPos,
  
  pub spinies: Vec<AnimatedPos>,
}

pub fn initial_state() -> State {
  State {
    time: 0.0,
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
    level_number: 0,
    
    up_pressed:    false,
    left_pressed:  false,
    down_pressed:  false,
    right_pressed: false,
    most_recent_dir: None,
    buffered_dir:    None,
    player: AnimatedPos::Idle([0, 1]),
    
    spinies: Vec::new(),
  }
}
