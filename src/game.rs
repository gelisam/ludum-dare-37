use graphics::math::*;

use corpse::*;
use player::*;
use spiny::*;
use state::*;
use types::*;
use types::RawInputEvent::*;
use types::AnimatedPos::*;


fn should_die(player: &AnimatedPos, spinies: &Vec<MovingPos>, t0: Seconds, t: Seconds) -> Option<Action> {
  let player_pos = compute_player_f_pos(player, t);
  let player_rect = compute_f_rect(player_pos);
  for spiny in spinies.iter() {
    let spiny_pos = compute_spiny_f_pos(spiny, t0, t);
    let spiny_rect = compute_f_rect(spiny_pos);
    
    if let Some(_) = overlap_rectangle(player_rect, spiny_rect) {
      return Some(Action::Die(player_pos));
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
          
          update_spinies(&mut state.spinies, state.level_number, &mut state.spinies_moving_since, t);
          update_corpses(&mut state.corpses, t);
          let player_action = update_player(&mut state.player, state.level_number, t);
          
          should_die(&state.player.pos, &state.spinies, state.spinies_moving_since, t).or(player_action)
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
    Die(f_pos) => {
      let corpse = Corpse {
        f_pos: f_pos,
        t0: state.time,
      };
      state.corpses.push_back(corpse);
      
      state.player.pos = MovingInUntil(state.level_number - 1, state.level_number, state.time);
    },
    
    PreviousLevel => {
      state.player.pos = MovingOutSince(state.level_number, state.level_number - 1, state.time);
    },
    NextLevel => {
      state.player.pos = MovingOutSince(state.level_number, state.level_number + 1, state.time);
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
