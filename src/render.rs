extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::*;
use graphics::math::{ Matrix2d };
use opengl_graphics::{ GlGraphics, Texture };

use font::*;
use levels::*;
use resources::*;
use types::*;


pub const PIXEL_SIZE: u8 = 5;

fn draw_image(texture: &Texture, transform: Matrix2d, gl: &mut GlGraphics) {
  unsafe {
    // Sharp pixels please!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    
    // No over-saturation please!
    gl::Disable(gl::FRAMEBUFFER_SRGB);
  }
  
  image(texture, transform, gl);
  
  unsafe {
    // Sometimes the pixels still aren't sharp. There is no logical reason why setting this again after the
    // image has already been drawn should help, but it does!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
  }
}

fn draw_cell(level_number: LevelNumber, pos: Pos, resource: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  use levels::CellDescription::*;
  
  match cell_at(level_number, pos) {
    Floor      => draw_image(&resource.floor,    transform, gl),
    LeftDoor   => draw_image(&resource.start,    transform, gl),
    RigthDoor  => draw_image(&resource.goal,     transform, gl),
    Key(_)     => draw_image(&resource.key,      transform, gl),
    LockedDoor => draw_image(&resource.locked,   transform, gl),
    OpenedDoor => draw_image(&resource.unlocked, transform, gl),
    Sign(_)    => draw_image(&resource.sign,     transform, gl),
    Spiny(_)   => draw_image(&resource.spiny,    transform, gl),
    Wall       => draw_image(&resource.wall,     transform, gl),
  }
}

fn draw_level(level_number: LevelNumber, resources: &Resources, transform: Matrix2d, gl: &mut GlGraphics) {
  for j in 0..LEVEL_HEIGHT {
    let dy = (j as f64) * SPRITE_HEIGHT;
    for i in 0..LEVEL_WIDTH {
      let dx = (i as f64) * SPRITE_WIDTH;
      
      draw_cell(level_number, [i,j], resources, transform.trans(dx, dy), gl);
    }
  }
}

pub fn render(state: &State, args: &piston::input::RenderArgs, resources: &Resources, gl: &mut GlGraphics) {
  gl.draw(args.viewport(), |c, gl| {
    clear([1.0, 1.0, 1.0, 1.0], gl);
    
    let transform = c.transform.scale(PIXEL_SIZE as f64, PIXEL_SIZE as f64);
    draw_level(state.level_number, resources, transform, gl);
    
    for message in state.message {
      // Fade to white to make the text more readable
      rectangle([1.0, 1.0, 1.0, 0.8], [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      
      draw_text(message, &resources.big_font, c.transform, gl);
    }
  });
}
