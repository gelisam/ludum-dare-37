use types::*;


pub struct LevelDescription {
  ascii_map: &'static str,
  signs: &'static [Message],
}

pub enum CellDescription {
  Floor,
  LeftDoor,
  RigthDoor,
  Key(Lifetime),
  LockedDoor,
  OpenedDoor,
  Sign(Message),
  Spiny(Dir),
  Wall,
}


pub const LEVEL_WIDTH: i8 = 9;
pub const LEVEL_HEIGHT: i8 = 7;

pub const LEVELS: [LevelDescription; 1] = [
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            vv##.\
                .##              ##.\
                .##      S0>>    ##.\
                .##>>          <<##.\
                .##              RD.\
                .##################.",
    signs: &[
      "This is a sequel to my Ludum Dare 31\n\
       game, \"I've Seen This Room Before\"."
    ],
  },
];


const DOT_WIDTH: usize = 1;
const CELL_WIDTH: usize = 2;
const ASCII_MAP_WIDTH: usize = DOT_WIDTH + (LEVEL_WIDTH as usize)*CELL_WIDTH + DOT_WIDTH;
const ASCII_MAP_OFFSET: usize = ASCII_MAP_WIDTH + DOT_WIDTH;

pub fn cell_at(level_number: LevelNumber, pos: Pos) -> CellDescription {
  use self::CellDescription::*;
  
  if (pos[0] < 0) || (pos[1] < 0) || (pos[0] >= LEVEL_WIDTH) || (pos[1] >= LEVEL_HEIGHT) {
    Floor
  } else {
    let level_description: &LevelDescription = &LEVELS[level_number as usize];
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
      (' ',' ') => Floor,
      ('L','D') => LeftDoor,
      ('R','D') => RigthDoor,
      ('K', _ ) => Key(number()),
      ('D','D') => LockedDoor,
      ('S', _ ) => Sign(level_description.signs.iter().nth(number() as usize).unwrap()),
      ('^','^') => Spiny(UP),
      ('<','<') => Spiny(LEFT),
      ('v','v') => Spiny(DOWN),
      ('>','>') => Spiny(RIGHT),
      ('#','#') => Wall,
      _         => panic!("syntax error in level description"),
    }
  }
}

pub fn load_spinies(level_number: LevelNumber) -> Vec<MovingPos> {
  use self::CellDescription::*;
  
  let mut vec = Vec::new();
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      let pos = [i,j];
      if let Spiny(dir) = cell_at(level_number, pos) {
        vec.push(MovingPos {pos: pos, dir: dir});
      }
    }
  }
  
  vec
}
