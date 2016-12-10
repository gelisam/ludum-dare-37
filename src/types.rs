pub enum RawInputEvent {
  MouseClick,
  TimePasses(f64),
}


pub struct State {
  pub time: f64,
  
  pub message: Option<&'static str>,
  pub square_rotation: f64,
}

pub fn initial_state() -> State {
  State {
    time: 0.0,
    message: Some("I've Seen This Room\n\
                   Twice Already"),
    square_rotation: 0.0,
  }
}
