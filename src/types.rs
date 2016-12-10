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
    message: Some(".............................................\n\
                   .                                           .\n\
                   .            I've Seen This Room            .\n\
                   .               Twice Already               .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .    made in 48 hours by Samuel Gélineau    .\n\
                   .             for Ludum Dare 37             .\n\
                   .                                           .\n\
                   .              theme: One Room              .\n\
                   .                                           .\n\
                   .                                           .\n\
                   .        press any arrow key to begin       .\n\
                   .                                           .\n\
                   ............................................."),
    level_number: 0,
  }
}
