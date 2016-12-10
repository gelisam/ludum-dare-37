use types::*;
use types::RawInputEvent::*;


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  match raw_input_event {
    TimePasses(dt) => {
      state.time += dt;
      
      state.square_rotation = 2.0 * state.time;
    },
    MouseClick => {
      state.message = None;
    },
  }
}
