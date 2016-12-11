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
mod levels;
mod render;
mod resources;
mod types;

use game::*;
use levels::*;
use render::*;
use resources::*;
use types::*;


fn main() {
  // Change this to OpenGL::V2_1 if not working.
  let opengl = OpenGL::V3_2;
  
  // Create an Glutin window.
  let mut window: Window = WindowSettings::new(
        "I've Seen This Room Twice Already",
        [ LEVEL_WIDTH  as u32 * PIXEL_SIZE as u32 * SPRITE_WIDTH
        , LEVEL_HEIGHT as u32 * PIXEL_SIZE as u32 * SPRITE_HEIGHT
        ]
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
    use piston::input::Input::{ Press, Release };
    use piston::input::keyboard::Key::{ Up, Left, Down, Right,  W, A, S, D,  H, J, K, L,  P, Space };
    use types::RawInputEvent::{ TimePasses, Move, Pause, AnyKey };
    
    match e {
      Render(args)                  => render(&mut state, &args, &resources, &mut gl),
      Update(args)                  => update(&mut state, TimePasses(args.dt)),
      
      // arrow keys
      Input(Press(Keyboard(Up)))    => update(&mut state, Move(UP)),
      Input(Press(Keyboard(Left)))  => update(&mut state, Move(LEFT)),
      Input(Press(Keyboard(Down)))  => update(&mut state, Move(DOWN)),
      Input(Press(Keyboard(Right))) => update(&mut state, Move(RIGHT)),
      
      // WASD controls
      Input(Press(Keyboard(W)))     => update(&mut state, Move(UP)),
      Input(Press(Keyboard(A)))     => update(&mut state, Move(LEFT)),
      Input(Press(Keyboard(S)))     => update(&mut state, Move(DOWN)),
      Input(Press(Keyboard(D)))     => update(&mut state, Move(RIGHT)),
      
      // vim controls
      Input(Press(Keyboard(K)))     => update(&mut state, Move(UP)),
      Input(Press(Keyboard(H)))     => update(&mut state, Move(LEFT)),
      Input(Press(Keyboard(J)))     => update(&mut state, Move(DOWN)),
      Input(Press(Keyboard(L)))     => update(&mut state, Move(RIGHT)),
      
      // pause
      Input(Press(Keyboard(P)))     => update(&mut state, Pause),
      Input(Press(Keyboard(Space))) => update(&mut state, Pause),
      
      // unpause
      Input(Release(Keyboard(_)))   => update(&mut state, AnyKey),
      
      _                             => ()
    }
  }
}
