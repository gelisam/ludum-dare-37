use graphics::math::*;

use types::*;
use types::RawInputEvent::*;


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  match state.message {
    Some(_) =>
      match raw_input_event {
        Pause | AnyKey => {
          state.message = None;
        },
        _ => {},
      },
    None =>
      match raw_input_event {
        TimePasses(dt) => {
          state.time += dt;
        },
        Move(dir) => {
          state.pos = add(state.pos, dir);
        },
        Pause => {
          state.message = Some(".............................................\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                ** PAUSED **               .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .                                           .\n\
                                .         press any key to continue         .\n\
                                .                                           .\n\
                                .............................................");
        }
        _ => {},
      },
  }
}
