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
  Move(Dir),
  TimePasses(Seconds),
}


pub struct State {
  pub time: Seconds,
  
  pub message: Option<Message>,
  pub level_number: LevelNumber,
}

pub fn initial_state() -> State {
  State {
    time: 0.0,
    message: Some("I've Seen This Room\n\
                   Twice Already"),
    level_number: 0,
  }
}
