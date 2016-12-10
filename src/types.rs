#[derive(Clone)]
pub struct Context {
  pub square_rotation: f64,
}


#[derive(Clone)]
pub enum RawInputEvent {
  MouseClick,
}


#[derive(Clone)]
pub struct State {
  pub is_square_activated: bool,
}
