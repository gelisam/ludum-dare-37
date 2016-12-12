use std::cmp;
use types::*;


pub struct LevelDescription {
  ascii_map: &'static str,
  signs: &'static [Message],
}

// Static information about a cell obtained straight from the level description
pub enum Cell {
  FloorC,
  LeftDoorC,
  RightDoorC,
  SignC(Message),
  SpinyC(Dir, Lifetime),
  WallC(Lifetime),
}

// Anything we need to keep track of in addition to the Cell contents, either because it moves, can
// be picked up, or just because we need to display it lifetime. So even walls are "alive" in this game.
pub enum Live {
  SpinyL(Dir, Lifetime),
  WallL(Lifetime),
}

pub fn lifetime_of(live: &Live) -> &Lifetime {
  use self::Live::*;
  
  match live {
    &SpinyL(_, ref lifetime) => lifetime,
    &WallL(ref lifetime)     => lifetime,
  }
}


pub const LEFT_DOOR:  Pos = [0, 1];
pub const RIGHT_DOOR: Pos = [8, 5];

pub const LEVEL_WIDTH: i8 = 9;
pub const LEVEL_HEIGHT: i8 = 7;

pub const LEVELS: [LevelDescription; 3] = [
  // LEVEL 1
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD      #1      ##.\
                .##              ##.\
                .##      S0      ##.\
                .##              ##.\
                .##              RD.\
                .##################.",
    signs: &[
      "This is a sequel to my Ludum Dare 31\n\
       game, \"I've Seen This Room Before\"."
    ],
  },
  
  // LEVEL 2
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD        >0>1  ##.\
                .##  ^0        ^0##.\
                .##  ##  S0    ####.\
                .##>0            ##.\
                .##              RD.\
                .##################.",
    signs: &[
      "Only slightly harder than the previous level :)"
    ],
  },
  
  // LEVEL 3
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD              ##.\
                .##              ##.\
                .##      S0      ##.\
                .##              ##.\
                .##              RD.\
                .##################.",
    signs: &[
      "Last level."
    ],
  },
];

pub fn min_level() -> LevelNumber {
  1
}

pub fn max_level() -> LevelNumber {
  LEVELS.len() as u8
}


const DOT_WIDTH: usize = 1;
const CELL_WIDTH: usize = 2;
const ASCII_MAP_WIDTH: usize = DOT_WIDTH + (LEVEL_WIDTH as usize)*CELL_WIDTH + DOT_WIDTH;
const ASCII_MAP_OFFSET: usize = ASCII_MAP_WIDTH + DOT_WIDTH;

pub fn cell_at(level_number: LevelNumber, pos: Pos) -> Cell {
  use self::Cell::*;
  use types::Lifetime::*;
  
  if (pos[0] < 0) || (pos[1] < 0) || (pos[0] >= LEVEL_WIDTH) || (pos[1] >= LEVEL_HEIGHT) {
    FloorC
  } else {
    let level_index = level_number as usize - 1; // LEVELS is 0-based, but level numbers are 1-based
    let level_description: &LevelDescription = &LEVELS[level_index];
    let (u1, u2) = {
          let x_index = pos[0] as usize;
          let y_index = pos[1] as usize;
          let char_index = ASCII_MAP_OFFSET + y_index*ASCII_MAP_WIDTH + x_index*CELL_WIDTH;
          
          ( level_description.ascii_map.as_bytes()[char_index]
          , level_description.ascii_map.as_bytes()[char_index + 1]
          )
        };
    
    // The second character is sometimes a number, e.g. "S0" and "S1" to distinguish two signs
    // in the same level, or "K2" for a key which is valid for levels CURRENT to (CURRENT+2).
    let number = || u2 - '0' as u8;
    
    match (u1 as char, u2 as char) {
      (' ',' ') => FloorC,
      ('L','D') => LeftDoorC,
      ('R','D') => RightDoorC,
      ('S', _ ) => SignC(level_description.signs.iter().nth(number() as usize).unwrap()),
      ('^','^') => SpinyC(UP,    Immortal),
      ('<','<') => SpinyC(LEFT,  Immortal),
      ('v','v') => SpinyC(DOWN,  Immortal),
      ('>','>') => SpinyC(RIGHT, Immortal),
      ('^', _ ) => SpinyC(UP,    Mortal(level_number, level_number + number())),
      ('<', _ ) => SpinyC(LEFT,  Mortal(level_number, level_number + number())),
      ('v', _ ) => SpinyC(DOWN,  Mortal(level_number, level_number + number())),
      ('>', _ ) => SpinyC(RIGHT, Mortal(level_number, level_number + number())),
      ('#','#') => WallC(Immortal),
      ('#', _ ) => WallC(Mortal(level_number, level_number + number())),
      _         => panic!("syntax error in level description"),
    }
  }
}

pub fn load_spinies(existing_spinies: Vec<MovingSpiny>, level_number: LevelNumber) -> Vec<MovingSpiny> {
  use self::Cell::*;
  use types::Lifetime::*;
  
  let mut vec = Vec::new();
  let mut lub = LevelNumber::max_value();
  for spiny in existing_spinies {
    if still_alive(&spiny.lifetime, level_number) {
      if let &Mortal(_, level_max) = &spiny.lifetime {
        lub = cmp::min(lub, level_max);
      }
      
      vec.push(spiny);
    }
  }
  
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      let pos = [i,j];
      if let SpinyC(dir, lifetime) = cell_at(level_number, pos) {
        if let &Mortal(_, level_max) = &lifetime {
          if level_max < lub {
            vec.push(MovingSpiny {pos: pos, dir: dir, lifetime: lifetime });
          }
        }
      }
    }
  }
  
  vec
}
