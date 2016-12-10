use types::*;
use types::RawInputEvent::*;


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  match raw_input_event {
    TimePasses(dt) => {
      state.time += dt;
    },
    Move(_) => {
      state.message = None;
    },
  }
}
