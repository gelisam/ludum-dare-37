use graphics::math::*;

use levels::*;
use types::*;
use types::RawInputEvent::*;
use types::PlayerPos::*;


pub const PLAYER_SPEED: f64 = 4.0; // cells per second
const TIME_TO_CROSS_CELL: f64 = 1.0 / PLAYER_SPEED;

fn try_move_action(level_number: LevelNumber, pos: Pos, dir: Dir) -> Option<Action> {
  use levels::CellDescription::*;
  use types::Action::*;
  
  match cell_at(level_number, add(pos, dir)) {
    LockedDoor => None,
    Sign(_)    => None,
    Wall       => None,
    _          => Some(Move(pos, dir)),
  }
}

fn initiate_move(state: &mut State, dir: Dir) -> Option<Action> {
  match state.player_pos {
    Idle(pos)       => try_move_action(state.level_number, pos, dir),
    MovingSince(..) => None,
  }
}

fn press_direction(is_pressed: &mut bool, buffered_dir: &mut Option<Dir>, most_recent_dir: &mut Option<Dir>, dir: Dir) {
  if !*is_pressed {
    // honor single-taps, but not the fake auto-repeat keypresses
    *buffered_dir = Some(dir);
  }
  
  *is_pressed = true;
  *most_recent_dir = Some(dir);
}

fn release_direction(is_pressed: &mut bool) {
  *is_pressed = false;
}

fn update_player_pos(state: &mut State, t: Seconds) -> Option<Action> {
  if let MovingSince(pos, dir, t0) = state.player_pos {
    if t >= t0 + TIME_TO_CROSS_CELL {
      state.player_pos = Idle(add(pos, dir));
      
      // If the user holds right and taps down, we want to go down one cell and then continue going right.
      if state.buffered_dir == Some(UP)    { return initiate_move(state, UP);    }
      if state.buffered_dir == Some(LEFT)  { return initiate_move(state, LEFT);  }
      if state.buffered_dir == Some(DOWN)  { return initiate_move(state, DOWN);  }
      if state.buffered_dir == Some(RIGHT) { return initiate_move(state, RIGHT); }
      
      // If the user is holding several keys, favour the most recent one.
      if state.up_pressed    && state.most_recent_dir == Some(UP)    { return initiate_move(state, UP);    }
      if state.left_pressed  && state.most_recent_dir == Some(LEFT)  { return initiate_move(state, LEFT);  }
      if state.down_pressed  && state.most_recent_dir == Some(DOWN)  { return initiate_move(state, DOWN);  }
      if state.right_pressed && state.most_recent_dir == Some(RIGHT) { return initiate_move(state, RIGHT); }
      
      // Continue moving in one of the pressed directions even if none is the most recent.
      if state.up_pressed    { return initiate_move(state, UP);    }
      if state.left_pressed  { return initiate_move(state, LEFT);  }
      if state.down_pressed  { return initiate_move(state, DOWN);  }
      if state.right_pressed { return initiate_move(state, RIGHT); }
    }
  }
  
  None
}


fn execute_action(state: &mut State, action: Action) {
  use types::Action::*;
  
  match action {
    Move(pos, dir) => {
      state.buffered_dir = None;
      state.player_pos = MovingSince(pos, dir, state.time);
    },
  }
}


pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  // Update the key statuses whether the game is paused or not, otherwise the character will keep moving
  // if the user pauses and then releases a key.
  match raw_input_event {
    PressUp      => press_direction(&mut state.up_pressed,    &mut state.buffered_dir, &mut state.most_recent_dir, UP),
    PressLeft    => press_direction(&mut state.left_pressed,  &mut state.buffered_dir, &mut state.most_recent_dir, LEFT),
    PressDown    => press_direction(&mut state.down_pressed,  &mut state.buffered_dir, &mut state.most_recent_dir, DOWN),
    PressRight   => press_direction(&mut state.right_pressed, &mut state.buffered_dir, &mut state.most_recent_dir, RIGHT),
    
    ReleaseUp    => release_direction(&mut state.up_pressed),
    ReleaseLeft  => release_direction(&mut state.left_pressed),
    ReleaseDown  => release_direction(&mut state.down_pressed),
    ReleaseRight => release_direction(&mut state.right_pressed),
    
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
          
          update_player_pos(state, t).map(|action| execute_action(state, action));
        },
        
        PressUp    => { initiate_move(state, UP   ).map(|action| execute_action(state, action)); },
        PressLeft  => { initiate_move(state, LEFT ).map(|action| execute_action(state, action)); },
        PressDown  => { initiate_move(state, DOWN ).map(|action| execute_action(state, action)); },
        PressRight => { initiate_move(state, RIGHT).map(|action| execute_action(state, action)); },
        
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
