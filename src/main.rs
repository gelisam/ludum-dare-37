extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::window::WindowSettings;

mod font;
mod game;
mod resources;
mod types;
mod dom;

use dom::*;
use game::*;
use resources::*;
use types::*;


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
  let mut state = initial_state();
  let resources = load_resources();
  let mut gl = GlGraphics::new(opengl);
  
  let mut events = window.events();
  while let Some(e) = events.next(&mut window) {
    use piston::input::Button::{ Mouse };
    use piston::input::Event::{ Render, Input, Update };
    use piston::input::Input::{ Press };
    use piston::input::MouseButton::{ Left };
    use types::RawInputEvent::{ MouseClick, TimePasses };
    
    match e {
      Render(args)              => render(&mut state, &mut gl, &args, &resources),
      Input(Press(Mouse(Left))) => update(&mut state, MouseClick),
      Update(args)              => update(&mut state, TimePasses(args.dt)),
      _                         => ()
    }
  }
}
