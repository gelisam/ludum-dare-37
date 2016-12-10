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
mod dom;

use frp::*;
use resources::*;
use dom::*;


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
