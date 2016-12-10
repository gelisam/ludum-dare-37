extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::math::{ Matrix2d };
use opengl_graphics::{ GlGraphics, Texture };

use font::*;
use levels::*;
use resources::*;
use types::*;


fn draw_image(texture: &Texture, transform: Matrix2d, gl: &mut GlGraphics) {
  use graphics::*;
  
  unsafe {
    // Sharp pixels please!
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
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

pub fn render(state: &State, args: &piston::input::RenderArgs, resources: &Resources, gl: &mut GlGraphics) {
  use graphics::*;
  
  let rotation = state.square_rotation;
  let (x, y) = ( (args.width / 2) as f64
               , (args.height / 2) as f64
               );
  
  gl.draw(args.viewport(), |c, gl| {
    // Clear the screen.
    clear([1.0, 1.0, 1.0, 1.0], gl);
    
    let transform = c.transform
        .trans(x, y)
        .rot_rad(rotation)
        .scale(5.0, 5.0)
        .trans(-5.0, -5.0);
    
    // Draw something rotating around the middle of the screen.
    draw_cell(state.level_number, [0,0], resources, transform, gl);
    
    for message in state.message {
      // Display the title over the animation
      rectangle([1.0, 1.0, 1.0, 0.5], [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      draw_text(message, &resources.big_font, c.transform, gl);
      draw_text("1-99", &resources.small_font, transform, gl);
    }
  });
}
