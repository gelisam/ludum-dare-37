use std::mem;
use graphics::math::*;

use corpse::*;
use levels::*;
use player::*;
use spiny::*;
use state::*;
use types::*;
use types::RawInputEvent::*;
use types::AnimatedPos::*;


fn should_die(player: &AnimatedPos, spinies: &Vec<MovingSpiny>, t0: Seconds, t: Seconds) -> Option<Action> {
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
  
  // you can't un-pause from the "THE END" message, you must quit using ESC
  if state.frozen {
    return;
  }
  
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
      
      state.player.pos = moving_in(state.previous_level, state.level_number, state.time);
    },
    
    PreviousLevel => {
      let next_level = state.level_number - 1;
      
      state.next_level = Some(next_level);
      state.player.pos = moving_out(state.level_number, next_level, state.time);
    },
    NextLevel => {
      let next_level = state.level_number + 1;
      
      state.next_level = Some(next_level);
      state.player.pos = moving_out(state.level_number, next_level, state.time);
    },
    TransitionLevel(level_src, level_dst) => {
      if level_dst < min_level() {
        *state = initial_state();
      } else if level_dst > max_level() as LevelNumber {
        state.message = Some(".............................................\n\
                              .                                           .\n\
                              .                                           .\n\
                              .                                           .\n\
                              .                  THE END                  .\n\
                              .                                           .\n\
                              .           Thank you for playing           .\n\
                              .    \"I've Seen This Room Twice Already\"!   .\n\
                              .                                           .\n\
                              .     If you have enjoyed it, consider      .\n\
                              .     playing the Ludum Dare 31 prequel,    .\n\
                              .     \"I've Seen This Room Before\" :)       .\n\
                              .                                           .\n\
                              .                                           .\n\
                              .             press esc to quit             .\n\
                              .                                           .\n\
                              .............................................");
        state.frozen = true;
      } else if state.time == state.spinies_moving_since { // only transition when the spinies are aligned with the grid
        state.player.pos = moving_in(level_src, level_dst, state.time);
        
        state.previous_level = level_src;
        state.level_number = level_dst;
        state.next_level = None;
        
        // I want to move the old spiny list into load_spinies so I can move some of the spinies into
        // the new spiny list, but we don't own it so I can't move it. Instead, I use mem::replace to
        // swap state.spinies with a dummy list I do own, then I move that one into load_spinies.
        let tmp = mem::replace(&mut state.spinies, Vec::new());
        state.spinies = adjust_spinies(tmp, level_src, level_dst);
        
        // Same for temporary_walls
        let tmp = mem::replace(&mut state.temporary_walls, Vec::new());
        state.temporary_walls = adjust_walls(tmp, level_src, level_dst);
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
