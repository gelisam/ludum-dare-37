use graphics::math::*;

use levels::*;
use types::*;
use types::AnimatedPos::*;


pub fn update_spiny(spiny: &mut AnimatedPos, level_number: LevelNumber, t: Seconds) {
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
