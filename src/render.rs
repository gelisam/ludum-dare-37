extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::*;
use graphics::math::{ Matrix2d, Vec2d };
use opengl_graphics::{ GlGraphics, Texture };

use font::*;
use game::*;
use levels::*;
use resources::*;
use types::*;


pub const PIXEL_SIZE: u8 = 5;

fn draw_sprite(texture: &Texture, f_pos: Vec2d<f64>, transform: Matrix2d, gl: &mut GlGraphics) {
  unsafe {
    // Sharp pixels please!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    
    // No over-saturation please!
    gl::Disable(gl::FRAMEBUFFER_SRGB);
  }
  
  let dx = f_pos[0] * SPRITE_WIDTH as f64;
  let dy = f_pos[1] * SPRITE_HEIGHT as f64;
  image(texture, transform.trans(dx, dy), gl);
  
  unsafe {
    // Sometimes the pixels still aren't sharp. There is no logical reason why setting this again after the
    // image has already been drawn should help, but it does!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
  }
}


fn draw_lower_cell(level_number: LevelNumber, pos: Pos, resource: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::CellDescription::*;
  
  let f_pos = [pos[0] as f64, pos[1] as f64];
  match cell_at(level_number, pos) {
    Floor      => draw_sprite(&resource.floor,    f_pos, transform, gl),
    LeftDoor   => draw_sprite(&resource.start,    f_pos, transform, gl),
    RigthDoor  => draw_sprite(&resource.goal,     f_pos, transform, gl),
    Key(_)     => draw_sprite(&resource.key,      f_pos, transform, gl),
    LockedDoor => draw_sprite(&resource.locked,   f_pos, transform, gl),
    OpenedDoor => draw_sprite(&resource.unlocked, f_pos, transform, gl),
    Sign(_)    => draw_sprite(&resource.sign,     f_pos, transform, gl),
    Spiny(_)   => draw_sprite(&resource.spiny,    f_pos, transform, gl),
    Wall       => draw_sprite(&resource.wall,     f_pos, transform, gl),
  }
}

fn draw_upper_cell(level_number: LevelNumber, pos: Pos, resource: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::CellDescription::*;
  
  let f_pos = [pos[0] as f64, pos[1] as f64];
  match cell_at(level_number, pos) {
    LeftDoor   => draw_sprite(&resource.start_top,    f_pos, transform, gl),
    RigthDoor  => draw_sprite(&resource.goal_top,     f_pos, transform, gl),
    OpenedDoor => draw_sprite(&resource.unlocked_top, f_pos, transform, gl),
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

fn draw_upper_level(level_number: LevelNumber, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for j in 0..LEVEL_HEIGHT {
    for i in 0..LEVEL_WIDTH {
      draw_upper_cell(level_number, [i,j], resources, transform, gl);
    }
  }
}


fn draw_player(state: &State, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use types::PlayerPos::*;
  
  let (pos, dir, dt) = match state.player_pos {
    Idle(pos)                 => (pos, [0, 0], 0.0),
    MovingSince(pos, dir, t0) => (pos, dir, state.time - t0),
  };
  
  let x = pos[0] as f64 + dt * PLAYER_SPEED * dir[0] as f64;
  let y = pos[1] as f64 + dt * PLAYER_SPEED * dir[1] as f64;
  
  draw_sprite(&resources.player, [x,y], transform, gl);
}

fn draw_characters(state: &State, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  draw_player(state, resources, transform, gl);
}


pub fn render(state: &State, args: &piston::input::RenderArgs, resources: &Resources, gl: &mut GlGraphics) {
  gl.draw(args.viewport(), |c, gl| {
    clear([1.0, 1.0, 1.0, 1.0], gl);
    
    let transform = c.transform.scale(PIXEL_SIZE as f64, PIXEL_SIZE as f64);
    draw_lower_level(state.level_number, resources, transform, gl);
    draw_characters(state, resources, transform, gl);
    draw_upper_level(state.level_number, resources, transform, gl);
    
    for message in state.message {
      // Fade to white to make the text more readable
      rectangle([1.0, 1.0, 1.0, 0.8], [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      
      draw_text(message, &resources.big_font, c.transform, gl);
    }
  });
}
