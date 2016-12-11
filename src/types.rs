use graphics::math::*;
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
pub type FSpeed = Vec2d<f64>;
pub type FRect = Rectangle<f64>;

pub fn linear_motion(pos: Pos, speed: FSpeed, t0: Seconds, t: Seconds) -> FPos {
  let dt = t - t0;
  
  let x = pos[0] as f64 + dt * speed[0];
  let y = pos[1] as f64 + dt * speed[1];
  
  [x,y]
}

pub fn compute_f_speed(dir: Dir, speed: f64) -> FSpeed {
  let dx = speed * dir[0] as f64;
  let dy = speed * dir[1] as f64;
  
  [dx,dy]
}

pub fn compute_f_pos(pos: Pos, dir: Dir, speed: f64, t0: Seconds, t: Seconds) -> FPos {
  linear_motion(pos, compute_f_speed(dir, speed), t0, t)
}

pub fn compute_f_rect(f_pos: FPos) -> FRect {
  // all sprites are 1 cell by 1 cell
  let f_rect = [f_pos[0], f_pos[1], 1.0, 1.0];
  
  // smaller collision rectangles to account for the round sprites
  margin_rectangle(f_rect, 0.15)
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
  PreviousLevel, NextLevel, TransitionLevel(LevelNumber, LevelNumber),
  Pause, Unpause,
}


pub enum AnimatedPos {
  Idle(Pos),
  MovingSince(Pos, Dir, Seconds),
  MovingOutSince(LevelNumber, LevelNumber, Seconds),
  MovingInUntil(LevelNumber, LevelNumber, Seconds),
}

pub fn moving_out(level_src: LevelNumber, level_dst: LevelNumber, t: Seconds) -> AnimatedPos {
  AnimatedPos::MovingOutSince(level_src, level_dst, t)
}

pub fn moving_in(level_src: LevelNumber, level_dst: LevelNumber, t: Seconds) -> AnimatedPos {
  AnimatedPos::MovingInUntil(level_src, level_dst, t + PLAYER_MOVE_DURATION)
}



pub struct MovingPos {
  pub pos: Pos,
  pub dir: Dir,
}
