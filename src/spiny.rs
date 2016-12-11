use std::collections::HashMap;
use graphics::math::*;

use levels::*;
use types::*;


fn should_bounce(
  spiny: &MovingPos,
  spinies_src: &HashMap<Pos, Dir>, // all the spinies, accessible by src.
  spinies_dst: &HashMap<Pos, i8>,  // all the spinies, accessible by dst.
  level_number: LevelNumber,
  t0: Seconds,
  t: Seconds
) -> bool {
  use levels::CellDescription::*;
  
  let src = spiny.pos;
  let dst = add(src, spiny.dir);
  
  // Spinies move in unison, that is, they are cell-aligned on the same frame and they are the same
  // fraction of the way to the next cell as every other spiny. This simplifies collisions.
  let dt = t - t0;
  
  // Case 0: collision with a wall
  //   .................
  //   .       .       .
  //   . >>>>> . ##### .
  //   . >>>>> . ##### .
  //   . >>>>> . ##### .
  //   .       .       .
  //   .................
  // 
  // This can only happen at t0, otherwise we would have bounced already.
  match cell_at(level_number, dst) {
    LeftDoor | RigthDoor | LockedDoor | Sign(_) | Wall => return true,
    _                                                  => {},
  }
  
  // Case 1: collision with a spiny at a cell boundary
  // 
  //   .................  .................  .................           .................
  //   .       .       .  .       .       .  .       .       .           .       .       .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .           . >>>>> . >>>>> .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .  but not  . >>>>> . >>>>> .
  //   . >>>>> . <<<<< .  . >>>>> . ^^^^^ .  . >>>>> . vvvvv .           . >>>>> . >>>>> .
  //   .       .       .  .       .       .  .       .       .           .       .       .
  //   .................  .................  .................           .................
  // 
  // This can only happen at t0, otherwise we would have bounced already.
  if let Some(spiny2_dir) = spinies_src.get(&dst) {
    if spiny.dir != *spiny2_dir {
      return true
    }
  }
  
  // Case 2: collision with a spiny in the middle of a cell
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
  if dt > SPINY_HALF_MOVE_DURATION {
    if let Some(nb_overlapping_spinies) = spinies_dst.get(&dst) {
      if *nb_overlapping_spinies > 1 {
        return true
      }
    }
  }
  
  // Case 3: chain collision
  // 
  //   .........................  .........................
  //   .       .       .       .  .       .       .       .
  //   . >>>>> . >>>>> . ^^^^^ .  .     >>>>>   >>>>>     .
  //   . >>>>> . >>>>> . ^^^^^ .  .     >>>>>   >>>>>     .
  //   . >>>>> . >>>>> . ^^^^^ .  .     >>>>>   >>>>>     .
  //   .       .       .       .  .       .       . ^^^^^ .
  //   .........................  ..................^^^^^..
  //                                              . ^^^^^ .
  //                                              .       .
  //                                              .       .
  //                                              .       .
  //                                              .       .
  //                                              .........
  // 
  // If the spiny in front of us bounces onto an obstacle, we should bounce as well, otherwise we'll have a head-on
  // collision with it in the next frame and it will bounce back towards the obstacle.
  if let Some(spiny2_dir) = spinies_src.get(&dst) {
    if spiny.dir == *spiny2_dir {
      let spiny2 = MovingPos { pos: dst, dir: *spiny2_dir };
      if should_bounce(&spiny2, spinies_src, spinies_dst, level_number, t0, t) {
        return true;
      }
    }
  }
  
  false
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
  if t >= *t0 + SPINY_MOVE_DURATION {
    *t0 = t;
    
    // Keep moving in the same direction, we'll handle collisions in a moment.
    for spiny in spinies.iter_mut() {
      *spiny = MovingPos {
        pos: add(spiny.pos, spiny.dir),
        dir: spiny.dir
      };
    }
  }
  
  // Our spinies are on a grid, so we should be able to look up spinies by their position.
  let mut spinies_src = HashMap::with_capacity(spinies.len());
  for spiny in spinies.iter() {
    spinies_src.insert(spiny.pos, spiny.dir);
  }
  
  // But the spinies are also moving, so their position overlaps two cells: their source and destination positions.
  // If two spinies are about to collide head-on in the middle of a cell, they can have the same destination.
  let mut spinies_dst = HashMap::with_capacity(spinies.len());
  for spiny in spinies.iter() {
    let dst = add(spiny.pos, spiny.dir);
    *spinies_dst.entry(dst).or_insert(0) += 1;
  }
  
  // We now have everything we need to determine if a spiny should bounce.
  for spiny in spinies.iter_mut() {
    if should_bounce(spiny, &mut spinies_src, &mut spinies_dst, level_number, *t0, t) {
      bounce_spiny(spiny, *t0, t);
    }
  }
}
