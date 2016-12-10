use graphics::math::Vec2d;

pub type Pos = Vec2d<i8>;
pub type Dir = Vec2d<i8>;
pub type Seconds = f64;
pub type Radians = f64;

pub enum RawInputEvent {
  Move(Dir),
  TimePasses(Seconds),
}


pub struct State {
  pub time: Seconds,
  
  pub message: Option<&'static str>,
  pub square_rotation: Radians,
}

pub fn initial_state() -> State {
  State {
    time: 0.0,
    message: Some("I've Seen This Room\n\
                   Twice Already"),
    square_rotation: 0.0,
  }
}
