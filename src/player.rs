use graphics::math::*;

use levels::*;
use types::*;
use types::AnimatedPos::*;


pub struct Player {
  pub up_pressed:    bool,
  pub left_pressed:  bool,
  pub down_pressed:  bool,
  pub right_pressed: bool,
  pub most_recent_dir: Option<Dir>, // favour the last key if many are pressed
  pub buffered_dir:    Option<Dir>, // a key tap which hasn't been honored yet
  pub pos: AnimatedPos,
}


// floating point version of player.pos which takes movement into account, so the
// position can be in-between two cells.
pub fn compute_player_f_pos(player: &AnimatedPos, t: Seconds) -> FPos {
  let (pos, f_speed, t0) = match player {
        &Idle(pos)                 => (pos, [0.0, 0.0], t),
        &MovingSince(pos, dir, t0) => (pos, compute_f_speed(dir, PLAYER_SPEED), t0),
        &MovingOutSince(level_src, level_dst, t0) => {
           if level_dst < level_src {
             (LEFT_DOOR, [-PLAYER_SPEED, 0.5 * PLAYER_SPEED], t0)
           } else {
             (RIGHT_DOOR, [PLAYER_SPEED, -0.5 * PLAYER_SPEED], t0)
           }
        },
        &MovingInUntil(level_src, level_dst, t_dst) => {
           if level_dst < level_src {
             (RIGHT_DOOR, [-PLAYER_SPEED, 0.5 * PLAYER_SPEED], t_dst)
           } else {
             (LEFT_DOOR, [PLAYER_SPEED, -0.5 * PLAYER_SPEED], t_dst)
           }
        },
      };
  
  linear_motion(pos, f_speed, t0, t)
}
  


fn try_move_action(level_number: LevelNumber, pos: Pos, dir: Dir) -> Option<Action> {
  use levels::Cell::*;
  use types::Action::*;
  
  match cell_at(level_number, pos) {
    LeftDoorC  => {
      if dir == LEFT {
        return Some(PreviousLevel);
      }
    },
    RightDoorC => {
      if dir == RIGHT {
        return Some(NextLevel);
      }
    },
    _          => {},
  }
  
  match cell_at(level_number, add(pos, dir)) {
    SignC(message) => Some(ReadSign(message)),
    WallC          => None,
    _              => Some(Move(pos, dir)),
  }
}


pub fn initiate_move(player: &mut Player, level_number: LevelNumber, dir: Dir) -> Option<Action> {
  match player.pos {
    Idle(pos)       => try_move_action(level_number, pos, dir),
    _               => None,
  }
}

pub fn press_direction(is_pressed: &mut bool, buffered_dir: &mut Option<Dir>, most_recent_dir: &mut Option<Dir>, dir: Dir) {
  if !*is_pressed {
    // honor single-taps, but not the fake auto-repeat keypresses
    *buffered_dir = Some(dir);
  }
  
  *is_pressed = true;
  *most_recent_dir = Some(dir);
}

pub fn release_direction(is_pressed: &mut bool) {
  *is_pressed = false;
}


fn continue_moving(player: &mut Player, level_number: LevelNumber) -> Option<Action> {
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
  
  None
}

pub fn update_player(player: &mut Player, level_number: LevelNumber, t: Seconds) -> Option<Action> {
  match player.pos {
    MovingSince(pos, dir, t0) => {
      if t >= t0 + PLAYER_MOVE_DURATION {
        player.pos = Idle(add(pos, dir));
        
        continue_moving(player, level_number)
      } else {
        None
      }
    },
    MovingOutSince(level_src, level_dst, t0) => {
      if t >= t0 + PLAYER_MOVE_DURATION {
        let t_dst = t + PLAYER_MOVE_DURATION;
        player.pos = moving_in(level_src, level_dst, t_dst);
        
        Some(Action::TransitionLevel(level_src, level_dst))
      } else {
        None
      }
    },
    MovingInUntil(level_src, level_dst, t_dst) => {
      if t >= t_dst {
        let door_dst = if level_dst < level_src {
              RIGHT_DOOR
            } else {
              LEFT_DOOR
            };
        player.pos = Idle(door_dst);
        
        continue_moving(player, level_number)
      } else {
        None
      }
    },
    _ => None,
  }
}
