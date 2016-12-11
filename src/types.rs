use graphics::math::Vec2d;


pub type Dir = Vec2d<i8>;
pub const UP:    Dir = [ 0,-1];
pub const LEFT:  Dir = [-1, 0];
pub const DOWN:  Dir = [ 0, 1];
pub const RIGHT: Dir = [ 1, 0];


// cells per second
pub const PLAYER_SPEED: f64 = 4.0;
pub const SPINY_SPEED:  f64 = 8.0;

// time to cross cell
pub const PLAYER_MOVE_DURATION: f64 = 1.0 / PLAYER_SPEED;
pub const SPINY_MOVE_DURATION:  f64 = 1.0 / SPINY_SPEED;


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

pub struct MovingPos {
  pub pos: Pos,
  pub dir: Dir,
}
