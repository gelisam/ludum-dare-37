extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{ GlGraphics };

use font::*;
use resources::*;
use types::*;


pub fn render(state: &State, gl: &mut GlGraphics, args: &piston::input::RenderArgs, resources: &Resources) {
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
    
    // Draw the player rotating around the middle of the screen.
    unsafe {
      // Sharp pixels please!
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
    image(&resources.player, transform, gl);
    unsafe {
      // Sometimes the pixels still aren't sharp. There is no logical reason why setting this again after the
      // image has already been drawn should help, but it does!
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
    
    for message in state.message {
      // Display the title over the animation
      rectangle([1.0, 1.0, 1.0, 0.5], [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      draw_text(message, &resources.big_font, c.transform, gl);
      draw_text("1-99", &resources.small_font, transform, gl);
    }
  });
}
