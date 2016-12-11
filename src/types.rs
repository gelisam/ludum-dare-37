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

pub struct Player {
  pub up_pressed:    bool,
  pub left_pressed:  bool,
  pub down_pressed:  bool,
  pub right_pressed: bool,
  pub most_recent_dir: Option<Dir>, // favour the last key if many are pressed
  pub buffered_dir:    Option<Dir>, // a key tap which hasn't been honored yet
  pub pos: AnimatedPos,
}
