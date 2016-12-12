use std::collections::HashMap;
use std::collections::HashSet;
use graphics::math::*;

use levels::*;
use types::*;


pub fn compute_spiny_f_pos(spiny: &MovingSpiny, t0: Seconds, t: Seconds) -> FPos {
  if spiny.enabled {
    compute_f_pos(spiny.pos, spiny.dir, SPINY_SPEED, t0, t)
  } else {
    [spiny.pos[0] as f64, spiny.pos[1] as f64]
  }
}


fn should_bounce(
  src: Pos,
  dir: Dir,
  spinies_src: &HashMap<Pos, Dir>, // all the spinies, accessible by src.
  spinies_dst: &HashMap<Pos, i8>,  // all the spinies, accessible by dst.
  level_number: LevelNumber,
  t0: Seconds,
  t: Seconds
) -> bool {
  use levels::Cell::*;
  
  let dst = add(src, dir);
  
  // Spinies move in unison, that is, they are cell-aligned on the same frame and they are the same
  // fraction of the way to the next cell as every other spiny. This simplifies collisions.
  let dt = t - t0;
  
  // Case 0: collision with a wall
  //   .................  .........................
  //   .       .       .  .       .       .       .
  //   . >>>>> . ##### .  . ##### . >>>>> . ##### .
  //   . >>>>> . ##### .  . ##### . >>>>> . ##### .
  //   . >>>>> . ##### .  . ##### . >>>>> . ##### .
  //   .       .       .  .       .       .       .
  //   .................  .........................
  // 
  // We bounce at SPINY_HALF_MOVE_DURATION, not t0, in order to avoid a corner case when a spiny is stuck between
  // two walls.
  if dt > SPINY_HALF_MOVE_DURATION {
    match cell_at(level_number, dst) {
      LeftDoorC | RightDoorC | SignC(_) | WallC => return true,
      _                                         => {},
    }
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
  if let Some(dir2) = spinies_src.get(&dst) {
    if dir != *dir2 {
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
  if let Some(dir2) = spinies_src.get(&dst) {
    if dir == *dir2 {
      if should_bounce(dst, *dir2, spinies_src, spinies_dst, level_number, t0, t) {
        return true;
      }
    }
  }
  
  false
}

fn bounce_spiny(spiny: &mut MovingSpiny, t0: Seconds, t: Seconds) {
  let dt = t - t0;
  
  spiny.pos = if dt <= SPINY_HALF_MOVE_DURATION {
                // Case 1: collision at a cell boundary
                spiny.pos
              } else {
                // Case 2: collision in the middle of a cell
                add(spiny.pos, spiny.dir)
              };
  spiny.dir = mul_scalar(spiny.dir, -1);
}

pub fn update_spinies(spinies: &mut Vec<MovingSpiny>, level_number: LevelNumber, t0: &mut Seconds, t: Seconds) {
  if t >= *t0 + SPINY_MOVE_DURATION {
    *t0 = t;
    
    // Keep moving in the same direction, we'll handle collisions in a moment.
    for spiny in spinies.iter_mut() {
      if spiny.enabled {
        spiny.pos = add(spiny.pos, spiny.dir);
      }
    }
    
    // re-enable spinies if appropriate
    let mut occupied = HashSet::new();
    for spiny in spinies.iter_mut() {
      if occupied.contains(&spiny.pos) {
        spiny.enabled = false;
      } else {
        occupied.insert(&spiny.pos);
        spiny.enabled = true;
      }
    }
  }
  
  // Our spinies are on a grid, so we should be able to look up spinies by their position.
  let mut spinies_src = HashMap::with_capacity(spinies.len());
  for spiny in spinies.iter() {
    if spiny.enabled {
      spinies_src.insert(spiny.pos, spiny.dir);
    }
  }
  
  // But the spinies are also moving, so their position overlaps two cells: their source and destination positions.
  // If two spinies are about to collide head-on in the middle of a cell, they can have the same destination.
  let mut spinies_dst = HashMap::with_capacity(spinies.len());
  for spiny in spinies.iter() {
    if spiny.enabled {
      let dst = add(spiny.pos, spiny.dir);
      *spinies_dst.entry(dst).or_insert(0) += 1;
    }
  }
  
  // We now have everything we need to determine if a spiny should bounce.
  for spiny in spinies.iter_mut() {
    if spiny.enabled && should_bounce(spiny.pos, spiny.dir, &mut spinies_src, &mut spinies_dst, level_number, *t0, t) {
      bounce_spiny(spiny, *t0, t);
    }
  }
}
