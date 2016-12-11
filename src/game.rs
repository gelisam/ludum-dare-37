use graphics::math::*;

use levels::*;
use types::*;
use types::RawInputEvent::*;
use types::AnimatedPos::*;


// cells per second
pub const PLAYER_SPEED: f64 = 4.0;
pub const SPINY_SPEED:  f64 = 8.0;

// time to cross cell
const PLAYER_MOVE_DURATION: f64 = 1.0 / PLAYER_SPEED;
const SPINY_MOVE_DURATION:  f64 = 1.0 / SPINY_SPEED;

fn update_spiny(spiny: &mut AnimatedPos, level_number: LevelNumber, t: Seconds) {
  use levels::CellDescription::*;
  
  if let MovingSince(pos, dir, t0) = *spiny {
    if t >= t0 + SPINY_MOVE_DURATION {
      let pos = add(pos, dir);
      *spiny = match cell_at(level_number, add(pos, dir)) {
        LockedDoor | Sign(_) | Wall => MovingSince(pos, mul_scalar(dir, -1), t), // bounce
        _                           => MovingSince(pos, dir,                 t), // keep moving
      }
    }
  }
}


fn try_move_action(level_number: LevelNumber, pos: Pos, dir: Dir) -> Option<Action> {
  use levels::CellDescription::*;
  use types::Action::*;
  
  match cell_at(level_number, add(pos, dir)) {
    LockedDoor    => None,
    Sign(message) => Some(ReadSign(message)),
    Wall          => None,
    _             => Some(Move(pos, dir)),
  }
}

fn initiate_move(player: &mut Player, level_number: LevelNumber, dir: Dir) -> Option<Action> {
  match player.pos {
    Idle(pos)       => try_move_action(level_number, pos, dir),
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

fn update_player(player: &mut Player, level_number: LevelNumber, t: Seconds) -> Option<Action> {
  if let MovingSince(pos, dir, t0) = player.pos {
    if t >= t0 + PLAYER_MOVE_DURATION {
      player.pos = Idle(add(pos, dir));
      
      // If the user holds right and taps down, we want to go down one cell and then continue going right.
      if player.buffered_dir == Some(UP)    { return initiate_move(player, level_number, UP);    }
      if player.buffered_dir == Some(LEFT)  { return initiate_move(player, level_number, LEFT);  }
      if player.buffered_dir == Some(DOWN)  { return initiate_move(player, level_number, DOWN);  }
      if player.buffered_dir == Some(RIGHT) { return initiate_move(player, level_number, RIGHT); }
      
      // If the user is holding several keys, favour the most recent one.
      if player.up_pressed    && player.most_recent_dir == Some(UP)    { return initiate_move(player, level_number, UP);    }
      if player.left_pressed  && player.most_recent_dir == Some(LEFT)  { return initiate_move(player, level_number, LEFT);  }
      if player.down_pressed  && player.most_recent_dir == Some(DOWN)  { return initiate_move(player, level_number, DOWN);  }
      if player.right_pressed && player.most_recent_dir == Some(RIGHT) { return initiate_move(player, level_number, RIGHT); }
      
      // Continue moving in one of the pressed directions even if none is the most recent.
      if player.up_pressed    { return initiate_move(player, level_number, UP);    }
      if player.left_pressed  { return initiate_move(player, level_number, LEFT);  }
      if player.down_pressed  { return initiate_move(player, level_number, DOWN);  }
      if player.right_pressed { return initiate_move(player, level_number, RIGHT); }
    }
  }
  
  None
}

fn handle_raw_input_event(state: &mut State, raw_input_event: RawInputEvent) -> Option<Action> {
  use types::Action::*;
  
  // Update the key statuses whether the game is paused or not, otherwise the character will keep moving
  // if the user pauses and then releases a key.
  match raw_input_event {
    PressUp      => press_direction(&mut state.player.up_pressed,    &mut state.player.buffered_dir, &mut state.player.most_recent_dir, UP),
    PressLeft    => press_direction(&mut state.player.left_pressed,  &mut state.player.buffered_dir, &mut state.player.most_recent_dir, LEFT),
    PressDown    => press_direction(&mut state.player.down_pressed,  &mut state.player.buffered_dir, &mut state.player.most_recent_dir, DOWN),
    PressRight   => press_direction(&mut state.player.right_pressed, &mut state.player.buffered_dir, &mut state.player.most_recent_dir, RIGHT),
    
    ReleaseUp    => release_direction(&mut state.player.up_pressed),
    ReleaseLeft  => release_direction(&mut state.player.left_pressed),
    ReleaseDown  => release_direction(&mut state.player.down_pressed),
    ReleaseRight => release_direction(&mut state.player.right_pressed),
    
    _ => {},
  }
  
  match state.message {
    Some(_) =>
      match raw_input_event {
        PressPause | PressAnyKey | ReleaseUp | ReleaseLeft | ReleaseDown | ReleaseRight => Some(Unpause),
        _                                                                               => None,
      },
    None =>
      match raw_input_event {
        TimePasses(dt) => {
          state.time += dt;
          let t = state.time;
          
          for spiny in &mut state.spinies {
            update_spiny(spiny, state.level_number, t);
          }
          update_player(&mut state.player, state.level_number, t)
        },
        
        PressUp    => initiate_move(&mut state.player, state.level_number, UP   ),
        PressLeft  => initiate_move(&mut state.player, state.level_number, LEFT ),
        PressDown  => initiate_move(&mut state.player, state.level_number, DOWN ),
        PressRight => initiate_move(&mut state.player, state.level_number, RIGHT),
        
        PressPause => Some(Pause),
        _          => None,
      },
  }
}

fn execute_action(state: &mut State, action: Action) {
  use types::Action::*;
  
  match action {
    Move(pos, dir) => {
      state.player.buffered_dir = None;
      state.player.pos = MovingSince(pos, dir, state.time);
    },
    ReadSign(message) => {
      state.message = Some(message);
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
    },
    Unpause => {
      state.message = None;
    },
  }
}

pub fn update(state: &mut State, raw_input_event: RawInputEvent) {
  if let Some(action) = handle_raw_input_event(state, raw_input_event) {
    execute_action(state, action);
  }
}
