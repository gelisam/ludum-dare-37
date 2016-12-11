use graphics::math::*;

use levels::*;
use types::*;


fn check_spiny_spiny_collision(spiny1: &MovingPos, spiny2: &MovingPos, t0: Seconds, t: Seconds) -> bool {
  // Spinies move in unison, that is, they are cell-aligned on the same frame and they are the same
  // fraction of the way to the next cell as every other spiny. This simplifies collisions.
  let dt = t - t0;
  
  // Case 1: collision at a cell boundary
  // 
  //   .................  .................  .................           .................
  //   .       .       .  .       .       .  .       .       .           .       .       .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .           . >>>>> . >>>>> .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .  but not  . >>>>> . >>>>> .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .           . >>>>> . >>>>> .
  //   .       .       .  .       .       .  .       .       .           .       .       .
  //   .................  .................  .................           .................
  // 
  // This can only happen at t0, otherwise spiny1 would have bounced already.
  if add(spiny1.pos, spiny1.dir) == spiny2.pos && spiny1.dir != spiny2.dir {
    true
  }
  
  // Case 2: collision in the middle of a cell
  // 
  //   .........................  .................           .........................  .................
  //   .       .       .       .  .       .       .           .       .       .       .  .       .       .
  //   .     >>>>>   <<<<<     .  .     >>>>>     .           .     >>>>>   >>>>>     .  .     >>>>>     .
  //   .     >>>>>   <<<<<     .  .     >>>>>     .  but not  .     >>>>>   >>>>>     .  .     >>>>>     .
  //   .     >>>>>   <<<<<     .  .     >>>>>     .           .     >>>>>   >>>>>     .  .     >>>>>     .
  //   .       .       .       .  .       . ^^^^^ .           .       .       .       .  .       . vvvvv .
  //   .........................  ..........^^^^^..           .........................  ..........vvvvv..
  //                                      . ^^^^^ .                                              . vvvvv .
  //                                      .       .                                              .       .
  //                                      .       .                                              .       .
  //                                      .       .                                              .       .
  //                                      .       .                                              .       .
  //                                      .........                                              .........
  // 
  // Already at t0 the collision is inevitable, but only the collision only happens after SPINY_HALF_MOVE_DURATION.
  else if dt > SPINY_HALF_MOVE_DURATION && add(spiny1.pos, spiny1.dir) == add(spiny2.pos, spiny2.dir) {
    true
  }
  
  else {
    false
  }
}

fn bounce_spiny(spiny: &mut MovingPos, t0: Seconds, t: Seconds) {
  *spiny = MovingPos {
    pos: if t == t0 {
      // Case 1: collision at a cell boundary
      spiny.pos
    } else {
      // Case 2: collision in the middle of a cell
      add(spiny.pos, spiny.dir)
    },
    dir: mul_scalar(spiny.dir, -1),
  }
}

pub fn update_spinies(spinies: &mut Vec<MovingPos>, level_number: LevelNumber, t0: &mut Seconds, t: Seconds) {
  use levels::CellDescription::*;
  
  if t >= *t0 + SPINY_MOVE_DURATION {
    *t0 = t;
    
    for spiny in &mut spinies[..] {
      let pos = add(spiny.pos, spiny.dir);
      *spiny = match cell_at(level_number, add(pos, spiny.dir)) {
        LockedDoor | Sign(_) | Wall => MovingPos { pos: pos, dir: mul_scalar(spiny.dir, -1) }, // bounce
        _                           => MovingPos { pos: pos, dir: spiny.dir,                }, // keep moving
      }
    }
  }
  
  let mut collisions = Vec::new();
  for (i, spiny1) in spinies.iter().enumerate() {
    for (j, spiny2) in spinies.iter().enumerate() {
      if i != j && check_spiny_spiny_collision(spiny1, spiny2, *t0, t) {
        collisions.push(i);
      }
    }
  }
  for i in collisions {
    bounce_spiny(&mut spinies[i], *t0, t);
  }
}
