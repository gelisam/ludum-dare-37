use graphics::math::*;

use types::*;
use types::RawInputEvent::*;
use types::PlayerPos::*;


pub const PLAYER_SPEED: f64 = 4.0; // cells per second
const TIME_TO_CROSS_CELL: f64 = 1.0 / PLAYER_SPEED;

fn initiate_move(state: &mut State, dir: Dir) {
  if let Idle(pos) = state.player_pos {
    state.player_pos = MovingSince(pos, dir, state.time);
  }
}

fn update_player_pos(state: &mut State, t: Seconds) {
  if let MovingSince(pos, dir, t0) = state.player_pos {
    if t >= t0 + TIME_TO_CROSS_CELL {
      state.player_pos = Idle(add(pos, dir));
      
      // If the user holds right and taps down, we want to go down one cell and then continue going right.
      if state.most_recent_dir == Some(UP)    { state.most_recent_dir = None; initiate_move(state, UP);    }
      if state.most_recent_dir == Some(LEFT)  { state.most_recent_dir = None; initiate_move(state, LEFT);  }
      if state.most_recent_dir == Some(DOWN)  { state.most_recent_dir = None; initiate_move(state, DOWN);  }
      if state.most_recent_dir == Some(RIGHT) { state.most_recent_dir = None; initiate_move(state, RIGHT); }
      
      // Continue moving if the key is held down.
      if state.up_pressed    { initiate_move(state, UP);    }
      if state.left_pressed  { initiate_move(state, LEFT);  }
      if state.down_pressed  { initiate_move(state, DOWN);  }
      if state.right_pressed { initiate_move(state, RIGHT); }
    }
  }
}


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  // Update the key statuses whether the game is paused or not, otherwise the character will keep moving
  // if the user pauses and then releases a key.
  match raw_input_event {
    PressUp    => { state.up_pressed    = true; state.most_recent_dir = Some(UP);    }
    PressLeft  => { state.left_pressed  = true; state.most_recent_dir = Some(LEFT);  }
    PressDown  => { state.down_pressed  = true; state.most_recent_dir = Some(DOWN);  }
    PressRight => { state.right_pressed = true; state.most_recent_dir = Some(RIGHT); }
    
    ReleaseUp    => state.up_pressed    = false,
    ReleaseLeft  => state.left_pressed  = false,
    ReleaseDown  => state.down_pressed  = false,
    ReleaseRight => state.right_pressed = false,
    
    _ => {},
  }
  
  match state.message {
    Some(_) =>
      match raw_input_event {
        Pause | AnyKey | ReleaseUp | ReleaseLeft | ReleaseDown | ReleaseRight => {
          state.message = None;
        },
        _ => {},
      },
    None =>
      match raw_input_event {
        TimePasses(dt) => {
          state.time += dt;
          let t = state.time;
          
          update_player_pos(state, t);
        },
        
        PressUp    => initiate_move(state, UP),
        PressLeft  => initiate_move(state, LEFT),
        PressDown  => initiate_move(state, DOWN),
        PressRight => initiate_move(state, RIGHT),
        
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
