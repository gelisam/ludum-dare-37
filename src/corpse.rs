use std::collections::VecDeque;

use types::*;


pub struct Corpse {
  pub f_pos: FPos,
  pub t0: Seconds,
}

fn can_remove_corpse(corpse: &Corpse, t: Seconds) -> bool {
  t >= corpse.t0 + CORPSE_FADE_OUT_DURATION
}

pub fn update_corpses(corpses: &mut VecDeque<Corpse>, t: Seconds) {
  loop {
    let can_remove = if let Some(corpse) = corpses.front() {
          can_remove_corpse(corpse, t)
        } else {
          false
        };
    
    if can_remove {
      corpses.pop_front();
    } else {
      break;
    }
  }
}
