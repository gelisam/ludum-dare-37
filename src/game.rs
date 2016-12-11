use types::*;
use types::RawInputEvent::*;


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  match state.message {
    Some(_) =>
      match raw_input_event {
        TimePasses(dt) => {
        },
        Move(dir) => {
          state.message = None;
        },
      },
    None =>
      match raw_input_event {
        TimePasses(dt) => {
          state.time += dt;
        },
        Move(dir) => {
        },
      },
  }
}
