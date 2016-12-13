use std::collections::HashSet;

use types::*;


pub struct LevelDescription {
  ascii_map: &'static str,
  signs: &'static [Message],
}

// Static information about a cell obtained straight from the level description
#[derive(PartialEq,Eq)]
pub enum Cell {
  FloorC,
  LeftDoorC,
  RightDoorC,
  SignC(Message),
  SpinyC(Dir),
  WallC,
}

// Anything we need to keep track of in addition to the Cell contents, either because it moves, can
// be picked up, or just because we need to display it lifetime.
pub enum Entity {
  SpinyE(MovingSpiny),
  WallE(TemporaryWall),
}


pub const LEFT_DOOR:  Pos = [0, 1];
pub const RIGHT_DOOR: Pos = [8, 5];

pub const LEVEL_WIDTH: i8 = 9;
pub const LEVEL_HEIGHT: i8 = 7;

pub fn is_inside_room(pos: Pos) -> bool {
  pos[0] > 0 && pos[1] > 0 && pos[0] < LEVEL_WIDTH-1 && pos[1] < LEVEL_HEIGHT-1
}

pub const LEVELS: &'static [LevelDescription; 13] = &[
  // LEVEL 1
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            ####.\
                .##              ##.\
                .##    ##S0##    ##.\
                .##              ##.\
                .##              RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "Can you figure out what the numbers mean?"
    ],
  },
  
  // LEVEL 2
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            ####.\
                .##              ##.\
                .##      S0####  ##.\
                .##              ##.\
                .##              RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "How about now?"
    ],
  },
  
  // LEVEL 3
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            ####.\
                .##              ##.\
                .##      S0  ##  ##.\
                .##          ##  ##.\
                .##              RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "You should be able to guess what's going to\n\
       happen to that corner block by now."
    ],
  },
  
  // LEVEL 4
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            S1##.\
                .##              ##.\
                .##          ^^  ##.\
                .##      ^^  ^^  ##.\
                .##S0^^  ^^  ^^  RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "I know, it's hard to read the numbers on the\n\
       spinies when they move so fast. But now that\n\
       you've stopped to read a sign, it's a lot\n\
       easier isn't it :)",
      "If there is no sign around, just press SPACE\n\
       or P to pause the game."
    ],
  },
  
  // LEVEL 5
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD            S0##.\
                .##              ##.\
                .##          ^^  ##.\
                .##      ^^  ##  ##.\
                .##  ^^  ##  ##  RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "Now you know!"
    ],
  },
  
  // LEVEL 6
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD              ##.\
                .##              ##.\
                .##      S0      ##.\
                .##  >>>>>>>>##  ##.\
                .##          S1  RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "Ready, set...",
      "GO!"
    ],
  },
  
  // LEVEL 7
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD  S0  S1  S2  ##.\
                .##              ##.\
                .##              ##.\
                .##  >>>>##>>>>  ##.\
                .##      S3      RD.\
                .##################.",
    signs: &[                                   //v-- last char
      "If you time yourself correctly, you can\n\
       control the number of spinies which appear\n\
       on either side.\n\
       \n\
       But how spinies can there be in a gap of\n\
       that size for you to be able to get through?",
      "Hint: the answer is in Level 5.",
      "Going to Level 5 will also re-synchronize\n\
       the spinies, which will make this easier.",
      "Congratulations! It's more luck than skill,\n\
       I know :("
    ],
  },
  
  // LEVEL 8
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
      "Almost there..."
    ],
  },
  
  // LEVEL 9
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD              ##.\
                .##          ##  ##.\
                .##          vv  ##.\
                .##>>>>>>>>>>    ##.\
                .##              RD.\
                .##################.",
    signs: &[
    ],
  },
  
  // LEVEL 10
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD          ##  ##.\
                .##          ##  ##.\
                .##          vv  ##.\
                .##  >>>>>>>>    ##.\
                .##              RD.\
                .##################.",
    signs: &[
    ],
  },
  
  // LEVEL 11
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
      "Same thing, with only one slight difference..."
    ],
  },
  
  // LEVEL 12
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD              ##.\
                .##          ##  ##.\
                .##          vv  ##.\
                .##vv>>>>>>>>    ##.\
                .##          ##  RD.\
                .##################.",
    signs: &[
    ],
  },
  
  // LEVEL 13
  LevelDescription {
    ascii_map: " . . . . . . . . . .\
                .##################.\
                .LD          ##  ##.\
                .##          ##  ##.\
                .##          vv  ##.\
                .##vv>>>>>>>>    ##.\
                .##              RD.\
                .##################.",
    signs: &[
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

fn get_level_description(level_number: LevelNumber) -> &'static LevelDescription {
  // LEVELS is 0-based, but level numbers are 1-based
  let level_index = level_number as usize - 1;
  
  &LEVELS[level_index]
}

fn message_at(level_number: LevelNumber, message_index: u8) -> Message {
  get_level_description(level_number).signs.iter().nth(message_index as usize).unwrap()
}

fn code_at(level_number: LevelNumber, pos: Pos) -> (u8, u8) {
  if (pos[0] < 0) || (pos[1] < 0) || (pos[0] >= LEVEL_WIDTH) || (pos[1] >= LEVEL_HEIGHT) {
    (' ' as u8, ' ' as u8)
  } else {
    let level_description: &LevelDescription = get_level_description(level_number);
    let x_index = pos[0] as usize;
    let y_index = pos[1] as usize;
    let char_index = ASCII_MAP_OFFSET + y_index*ASCII_MAP_WIDTH + x_index*CELL_WIDTH;
    
    let u1 = level_description.ascii_map.as_bytes()[char_index];
    let u2 = level_description.ascii_map.as_bytes()[char_index+1];
    
    (u1,u2)
  }
}

pub fn cell_at(level_number: LevelNumber, pos: Pos) -> Cell {
  use self::Cell::*;
  
  if (pos[0] < 0) || (pos[1] < 0) || (pos[0] >= LEVEL_WIDTH) || (pos[1] >= LEVEL_HEIGHT) {
    FloorC
  } else {
    let (u1,u2) = code_at(level_number, pos);
    
    // The second character is sometimes a number, e.g. "S0" and "S1" to distinguish two signs
    // in the same level, or "K2" for a key which is valid for levels CURRENT to (CURRENT+2).
    let number = || u2 - '0' as u8;
    
    match u1 as char {
      ' ' => FloorC,
      'L' => LeftDoorC,
      'R' => RightDoorC,
      'S' => SignC(message_at(level_number, number())),
      '^' => SpinyC(UP),
      '<' => SpinyC(LEFT),
      'v' => SpinyC(DOWN),
      '>' => SpinyC(RIGHT),
      '#' => WallC,
      _   => panic!("syntax error in level description"),
    }
  }
}

fn lifetime_at(level_number: LevelNumber, pos: Pos) -> Lifetime {
  let expected_cell = cell_at(level_number, pos);
  
  let mut level_min = level_number;
  while level_min > min_level() && cell_at(level_min - 1, pos) == expected_cell {
    level_min -= 1;
  }
  
  let mut level_max = level_number;
  while level_max < max_level() as u8 && cell_at(level_max + 1, pos) == expected_cell {
    level_max += 1;
  }
  
  Lifetime {
    level_min: level_min,
    level_max: level_max,
  }
}

pub fn entity_at(level_number: LevelNumber, pos: Pos) -> Option<Entity> {
  use self::Cell::*;
  use self::Entity::*;
  
  if (pos[0] < 0) || (pos[1] < 0) || (pos[0] >= LEVEL_WIDTH) || (pos[1] >= LEVEL_HEIGHT) {
    None
  } else {
    match cell_at(level_number, pos) {
      SpinyC(dir) => Some(
                       SpinyE(
                         MovingSpiny {
                           pos: pos,
                           dir: dir,
                           lifetime: lifetime_at(level_number, pos),
                           enabled: true,
                         }
                       )
                     ),
      WallC       => if is_inside_room(pos) {
                       Some(
                         WallE(
                           TemporaryWall {
                             pos: pos,
                             lifetime: lifetime_at(level_number, pos),
                           }
                         )
                       )
                     } else {
                       None
                     },
      _           => None,
    }
  }
}


// Operations on entities which work with both spinies and walls.

fn keep_live_entities<A, F>(lifetime_of_a: F, existing_entities: Vec<A>, level_number: LevelNumber) -> Vec<A>
  where F: Fn(&A) -> &Lifetime
{
  let mut remaining_entities = Vec::new();
  for entity in existing_entities {
    if still_alive(lifetime_of_a(&entity), level_number) {
      remaining_entities.push(entity);
    }
  }
  
  remaining_entities
}

fn list_entities<A, G>(is_a: &G, level_number: LevelNumber) -> Vec<A>
  where G: Fn(Entity) -> Option<A>
{
  let mut vec = Vec::new();
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      if let Some(entity) = entity_at(level_number, [i,j]).and_then(is_a) {
        vec.push(entity);
      }
    }
  }
  
  vec
}

// We only want to push new entities, that is, those which were dead in level_src and live in level_dst.
fn push_new_entities<A, F, G>(
  lifetime_of_a: &F,
  is_a: &G,
  entities: &mut Vec<A>,
  level_src: LevelNumber,
  level_dst: LevelNumber
)
  where F: Fn(&A) -> &Lifetime,
        G: Fn(Entity) -> Option<A>,
{
  for entity in list_entities(is_a, level_dst) {
    if !still_alive(lifetime_of_a(&entity), level_src) {
      entities.push(entity);
    }
  }
}

// Keep the entities which are still alive and add the new ones.
fn adjust_entities<A, F, G>(
  lifetime_of_a: &F,
  is_a: &G,
  existing_entities: Vec<A>,
  level_src: LevelNumber,
  level_dst: LevelNumber
) -> Vec<A>
  where F: Fn(&A) -> &Lifetime,
        G: Fn(Entity) -> Option<A>,
{
  let mut remaining_entities = keep_live_entities(lifetime_of_a, existing_entities, level_dst);
  push_new_entities(lifetime_of_a, is_a, &mut remaining_entities, level_src, level_dst);
  
  remaining_entities
}


// Helpers to make the entity operations above work with both spinies and walls.

fn lifetime_of_spiny(moving_spiny: &MovingSpiny) -> &Lifetime {
  &moving_spiny.lifetime
}

fn lifetime_of_wall(temporary_wall: &TemporaryWall) -> &Lifetime {
  &temporary_wall.lifetime
}

fn is_spiny(entity: Entity) -> Option<MovingSpiny> {
  use self::Entity::*;
  
  match entity {
    SpinyE(moving_spiny) => Some(moving_spiny),
    _                    => None,
  }
}

fn is_wall(entity: Entity) -> Option<TemporaryWall> {
  use self::Entity::*;
  
  match entity {
    WallE(temporary_wall) => Some(temporary_wall),
    _                     => None,
  }
}


// specializations of the entity operations for spinies and walls.

pub fn adjust_spinies(existing_spinies: Vec<MovingSpiny>, level_src: LevelNumber, level_dst: LevelNumber) -> Vec<MovingSpiny> {
  let mut spinies = adjust_entities(&lifetime_of_spiny, &is_spiny, existing_spinies, level_src, level_dst);
  
  // temporarily disable some spinies if they occupy the same cell
  {
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
  
  spinies
}

pub fn adjust_walls(existing_walls: Vec<TemporaryWall>, level_src: LevelNumber, level_dst: LevelNumber) -> Vec<TemporaryWall> {
  adjust_entities(&lifetime_of_wall, &is_wall, existing_walls, level_src, level_dst)
}
