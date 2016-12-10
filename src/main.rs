#[macro_use] extern crate carboxyl;
extern crate carboxyl_time;
extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use carboxyl::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::window::WindowSettings;

mod font;
mod frp;
mod resources;
mod types;
use font::*;
use frp::*;
use resources::*;
use types::*;


const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COVER: [f32; 4] = [1.0, 1.0, 1.0, 0.5];

fn render(gl: &mut GlGraphics, args: &piston::input::RenderArgs, resources: &Resources, context: &Context, state: &State) {
  use graphics::*;
  
  let rotation = context.square_rotation;
  let active = state.is_square_activated;
  let (x, y) = ( (args.width / 2) as f64
               , (args.height / 2) as f64
               );
  
  gl.draw(args.viewport(), |c, gl| {
    // Sharp pixels please!
    unsafe {
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
    
    // Clear the screen.
    clear(WHITE, gl);
    
    let transform = c.transform
        .trans(x, y)
        .rot_rad(rotation)
        .scale(5.0, 5.0)
        .trans(-5.0, -5.0);
    
    // Draw the player rotating around the middle of the screen.
    image(&resources.player, transform, gl);
    
    if !active {
      // Display the title over the animation
      rectangle(COVER, [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
      draw_text("I've Seen This Room\n\
                 Twice Already", &resources.big_font, c.transform, gl);
      draw_text("1-99", &resources.small_font, transform, gl);
    }
  });
}

fn main() {
  // Change this to OpenGL::V2_1 if not working.
  let opengl = OpenGL::V3_2;
  
  // Create an Glutin window.
  let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
      )
      .opengl(opengl)
      .exit_on_esc(true)
      .build()
      .unwrap();
  let mut gl = GlGraphics::new(opengl);
  let resources = load_resources();
  
  let sink = Sink::new();
  let (context, state) = frp_network(&sink.stream());
  
  let mut events = window.events();
  while let Some(e) = events.next(&mut window) {
    use piston::input::Button::{ Mouse };
    use piston::input::Event::{ Render, Input };
    use piston::input::Input::{ Press };
    use piston::input::MouseButton::{ Left };
    use types::RawInputEvent::{ MouseClick };
    
    match e {
      Render(args)              => render(&mut gl, &args, &resources, &context.sample(), &state.sample()),
      Input(Press(Mouse(Left))) => sink.send(MouseClick),
      _                         => ()
    }
  }
}
