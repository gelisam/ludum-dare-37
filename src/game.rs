use graphics::math::*;

use types::*;
use types::RawInputEvent::*;
use types::PlayerPos::*;


pub const PLAYER_SPEED: f64 = 8.0; // cells per second
const TIME_TO_CROSS_CELL: f64 = 1.0 / PLAYER_SPEED;

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
          let t = state.time;
          
          if let MovingSince(pos, dir, t0) = state.player_pos {
            if t >= t0 + TIME_TO_CROSS_CELL {
              state.player_pos = Idle(add(pos, dir));
            }
          }
        },
        Move(dir) => {
          match state.player_pos {
            Idle(pos) => state.player_pos = MovingSince(pos, dir, state.time),
            _         => {},
          }
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
