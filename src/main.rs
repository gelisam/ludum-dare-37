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
mod render;
mod resources;
mod types;

use game::*;
use render::*;
use resources::*;
use types::*;


fn main() {
  // Change this to OpenGL::V2_1 if not working.
  let opengl = OpenGL::V3_2;
  
  // Create an Glutin window.
  let mut window: Window = WindowSettings::new(
        "I've Seen This Room Twice Already",
        [512, 512]
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
    use piston::input::Button::{ Keyboard };
    use piston::input::Event::{ Render, Input, Update };
    use piston::input::Input::{ Press };
    use piston::input::keyboard::Key::{ Up, Left, Down, Right };
    use types::RawInputEvent::{ Move, TimePasses };
    
    match e {
      Render(args)                  => render(&mut state, &mut gl, &args, &resources),
      Input(Press(Keyboard(Up)))    => update(&mut state, Move([ 0,-1])),
      Input(Press(Keyboard(Left)))  => update(&mut state, Move([-1, 0])),
      Input(Press(Keyboard(Down)))  => update(&mut state, Move([ 0, 1])),
      Input(Press(Keyboard(Right))) => update(&mut state, Move([ 1, 0])),
      Update(args)                  => update(&mut state, TimePasses(args.dt)),
      _                             => ()
    }
  }
}
