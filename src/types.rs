use graphics::math::Vec2d;
use graphics::types::Rectangle;


pub type Dir = Vec2d<i8>;
pub const UP:    Dir = [ 0,-1];
pub const LEFT:  Dir = [-1, 0];
pub const DOWN:  Dir = [ 0, 1];
pub const RIGHT: Dir = [ 1, 0];


pub type Seconds = f64;

// cells per second
pub const PLAYER_SPEED: f64 = 4.0;
pub const SPINY_SPEED:  f64 = 8.0;

// time to cross cell
pub const PLAYER_MOVE_DURATION:     Seconds = 1.0 / PLAYER_SPEED;
pub const SPINY_MOVE_DURATION:      Seconds = 1.0 / SPINY_SPEED;
pub const SPINY_HALF_MOVE_DURATION: Seconds = SPINY_MOVE_DURATION / 2.0;

pub const CORPSE_FADE_OUT_DURATION: Seconds = 1.0;
pub const CORPSE_SPEED: f64 = 1.0 / CORPSE_FADE_OUT_DURATION; // fraction of alpha to remove per second


pub type Pos = Vec2d<i8>;
pub type FPos = Vec2d<f64>;
pub type FRect = Rectangle<f64>;

pub fn compute_f_pos(pos: Pos, dir: Dir, speed: f64, t0: Seconds, t: Seconds) -> FPos {
  let dt = t - t0;
  
  let x = pos[0] as f64 + dt * speed * dir[0] as f64;
  let y = pos[1] as f64 + dt * speed * dir[1] as f64;
  
  [x,y]
}

pub fn compute_f_rect(f_pos: FPos) -> FRect {
  // all sprites are 1 cell by 1 cell
  [f_pos[0], f_pos[1], 1.0, 1.0]
}


pub type Message = &'static str;
pub type LevelNumber = u8;
pub type Lifetime = u8;
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
  Die(FPos),
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
