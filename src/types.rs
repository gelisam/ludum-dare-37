pub enum RawInputEvent {
  MouseClick,
  TimePasses(f64),
}


pub struct State {
  pub time: f64,
  
  pub is_square_activated: bool,
  pub square_rotation: f64,
}

pub fn initial_state() -> State {
  State {
    time: 0.0,
    is_square_activated: false,
    square_rotation: 0.0,
  }
}
