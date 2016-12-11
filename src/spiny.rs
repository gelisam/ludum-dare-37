use graphics::math::*;

use levels::*;
use types::*;


pub fn update_spinies(spinies: &mut Vec<MovingPos>, level_number: LevelNumber, t0: &mut Seconds, t: Seconds) {
  use levels::CellDescription::*;
  
  if t >= *t0 + SPINY_MOVE_DURATION {
    *t0 = t;
    
    for spiny in spinies {
      let pos = add(spiny.pos, spiny.dir);
      *spiny = match cell_at(level_number, add(pos, spiny.dir)) {
        LockedDoor | Sign(_) | Wall => MovingPos { pos: pos, dir: mul_scalar(spiny.dir, -1) }, // bounce
        _                           => MovingPos { pos: pos, dir: spiny.dir,                }, // keep moving
      }
    }
  }
}
