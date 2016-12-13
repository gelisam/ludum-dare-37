extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::*;
use graphics::math::{ Matrix2d };
use opengl_graphics::{ GlGraphics, Texture };

use corpse::*;
use font::*;
use levels::*;
use player::*;
use resources::*;
use spiny::*;
use state::*;
use types::*;


pub const SPRITE_PIXEL_SIZE: u8 = 5;
pub const LIFETIME_PIXEL_SIZE: u8 = 2;

fn draw_transparent_sprite(texture: &Texture, f_pos: FPos, alpha: f64, transform: Matrix2d, gl: &mut GlGraphics) {
  unsafe {
    // Sharp pixels please!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    
    // No over-saturation please!
    gl::Disable(gl::FRAMEBUFFER_SRGB);
  }
  
  let color = [1.0, 1.0, 1.0, alpha as f32];
  let dx = f_pos[0] * SPRITE_WIDTH as f64;
  let dy = f_pos[1] * SPRITE_HEIGHT as f64;
  let xform = transform.scale(SPRITE_PIXEL_SIZE as f64, SPRITE_PIXEL_SIZE as f64)
                       .trans(dx, dy);
  Image::new_color(color).draw(texture, &Default::default(), xform, gl);
  
  unsafe {
    // Sometimes the pixels still aren't sharp. There is no logical reason why setting this again after the
    // image has already been drawn should help, but it does!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
  }
}

fn draw_sprite(texture: &Texture, f_pos: FPos, transform: Matrix2d, gl: &mut GlGraphics) {
  draw_transparent_sprite(texture, f_pos, 1.0, transform, gl);
}

fn draw_time_bound_sprite(
  texture: &Texture,
  f_pos: FPos,
  lifetime: &Lifetime,
  resources: &Resources,
  transform: Matrix2d,
  gl: &mut GlGraphics
) {
  draw_sprite(texture, f_pos, transform, gl);
  
  let lifetime_text = format!("{}-{}", lifetime.level_min, lifetime.level_max);
  let dx = (f_pos[0] + 1.0) * SPRITE_WIDTH as f64 * SPRITE_PIXEL_SIZE as f64;
  let dy = (f_pos[1] + 1.0) * SPRITE_HEIGHT as f64 * SPRITE_PIXEL_SIZE as f64;
  let xform = transform.trans(dx, dy)
                       .scale(LIFETIME_PIXEL_SIZE as f64, LIFETIME_PIXEL_SIZE as f64);
  draw_text_bottom_right(&lifetime_text, &resources.small_font, xform, gl);
}


fn draw_lower_cell(level_number: LevelNumber, pos: Pos, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::Cell::*;
  
  let f_pos = [pos[0] as f64, pos[1] as f64];
  match cell_at(level_number, pos) {
    LeftDoorC   => draw_sprite(&resources.start,    f_pos, transform, gl),
    RightDoorC  => draw_sprite(&resources.goal,     f_pos, transform, gl),
    SignC(_)    => draw_sprite(&resources.sign,     f_pos, transform, gl),
    _           => draw_sprite(&resources.floor,    f_pos, transform, gl),
  }
}

fn draw_static_entity(level_number: LevelNumber, pos: Pos, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::Entity::*;
  
  match entity_at(level_number, pos) {
    Some(SpinyE(moving_spiny))  => draw_spiny(&moving_spiny, 0.0, 0.0, &resources, transform, gl),
    Some(WallE(temporary_wall)) => draw_temporary_wall(&temporary_wall, &resources, transform, gl),
    _                           => {},
  }
}

fn draw_upper_cell(level_number: LevelNumber, pos: Pos, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::Cell::*;
  
  let f_pos = [pos[0] as f64, pos[1] as f64];
  match cell_at(level_number, pos) {
    LeftDoorC  => draw_sprite(&resources.start_top,    f_pos, transform, gl),
    RightDoorC => draw_sprite(&resources.goal_top,     f_pos, transform, gl),
    WallC      => if !is_inside_room(pos) {
                    // draw over the character, but not over the temporary walls
                    draw_sprite(&resources.wall, f_pos, transform, gl)
                  },
    _          => {},
  }
}

fn draw_lower_level(level_number: LevelNumber, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      draw_lower_cell(level_number, [i,j], resources, transform, gl);
    }
  }
}

fn draw_static_entities(level_number: LevelNumber, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      draw_static_entity(level_number, [i,j], resources, transform, gl);
    }
  }
}

fn draw_upper_level(level_number: LevelNumber, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      draw_upper_cell(level_number, [i,j], resources, transform, gl);
    }
  }
}


fn draw_player(player: &Player, t: Seconds, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  draw_sprite(&resources.player, compute_player_f_pos(&player.pos, t), transform, gl);
}

fn draw_corpse(corpse: &Corpse, t: Seconds, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  let dt = t - corpse.t0;
  let alpha = 1.0 - CORPSE_SPEED * dt;
  
  draw_transparent_sprite(&resources.player, corpse.f_pos, alpha, transform, gl);
}

fn draw_spiny(spiny: &MovingSpiny, t0: Seconds, t: Seconds, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  draw_time_bound_sprite(&resources.spiny, compute_spiny_f_pos(spiny, t0, t), &spiny.lifetime, resources, transform, gl);
}

fn draw_temporary_wall(temporary_wall: &TemporaryWall, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  let f_pos = [temporary_wall.pos[0] as f64, temporary_wall.pos[1] as f64];
  
  draw_time_bound_sprite(&resources.wall, f_pos, &temporary_wall.lifetime, resources, transform, gl);
}

fn draw_entities(state: &State, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for corpse in &state.corpses {
    draw_corpse(corpse, state.time, resources, transform, gl);
  }
  
  draw_player(&state.player, state.time, resources, transform, gl);
  
  for spiny in &state.spinies {
    draw_spiny(spiny, state.spinies_moving_since, state.time, resources, transform, gl);
  }
  
  for wall in &state.temporary_walls {
    draw_temporary_wall(wall, resources, transform, gl);
  }
}


pub fn render(state: &State, args: &piston::input::RenderArgs, resources: &Resources, gl: &mut GlGraphics) {
  gl.draw(args.viewport(), |c, gl| {
    clear([1.0, 1.0, 1.0, 1.0], gl);
    
    let transform = c.transform;
    
    let show_next_level = if state.time % TRANSITION_FLASH_DURATION < TRANSITION_HALF_FLASH_DURATION {
                            if let Some(next_level) = state.next_level {
                              if next_level <= max_level() {
                                Some(next_level)
                              } else {
                                None
                              }
                            } else {
                              None
                            }
                          } else {
                            None
                          };
    if let Some(next_level) = show_next_level {
      draw_lower_level(next_level, resources, transform, gl);
      draw_static_entities(next_level, resources, transform, gl);
      draw_upper_level(next_level, resources, transform, gl);
    } else {
      draw_lower_level(state.level_number, resources, transform, gl);
      draw_entities(state, resources, transform, gl);
      draw_upper_level(state.level_number, resources, transform, gl);
    }
    
    let level_text = format!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n Level {}", state.level_number);
    draw_text(&level_text, &resources.white_font, c.transform, gl);
    
    for message in state.message {
      // Fade to white to make the text more readable
      rectangle([1.0, 1.0, 1.0, 0.8], [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      
      draw_text(message, &resources.big_font, c.transform, gl);
    }
  });
}
