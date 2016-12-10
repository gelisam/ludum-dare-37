#[macro_use] extern crate carboxyl;
extern crate carboxyl_time;
extern crate gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate time;

use carboxyl::*;
use carboxyl_time::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::window::WindowSettings;


#[derive(Clone)]
struct Context {
  square_rotation: f64,
}


#[derive(Clone)]
enum RawInputEvent {
  MouseClick,
}


#[derive(Clone)]
struct State {
  is_square_activated: bool,
}


fn frp_network(raw_input_events: &Stream<RawInputEvent>) -> (Signal<Context>, Signal<State>) {
  let seconds: Signal<f64> = {
    let time: Signal<time::Tm> = now();
    let t0 = time.sample();
    lift!(move |t| (t - t0).num_milliseconds() as f64 / 1000.0, &time)
  };

  let square_rotation = lift!(|t| 2.0 * t, &seconds); // Rotate 2 radians per second.
  let is_square_activated = raw_input_events.fold(false, |b, _| !b); // Toggle on each click.

  let context = lift!(|r|
    Context {
      square_rotation: r,
    },
    &square_rotation
  );
  let state = lift!(|a|
    State {
      is_square_activated: a,
    },
    &is_square_activated
  );

  (context, state)
}


struct Resources {
  //big_font:      opengl_graphics::Texture,
  //floor:         opengl_graphics::Texture,
  //goal_top:      opengl_graphics::Texture,
  //goal:          opengl_graphics::Texture,
  //inventory_key: opengl_graphics::Texture,
  //key:           opengl_graphics::Texture,
  //locked:        opengl_graphics::Texture,
  player:        opengl_graphics::Texture,
  //sign:          opengl_graphics::Texture,
  //small_font:    opengl_graphics::Texture,
  //spiny:         opengl_graphics::Texture,
  //start_top:     opengl_graphics::Texture,
  //start:         opengl_graphics::Texture,
  //unlocked_top:  opengl_graphics::Texture,
  //unlocked:      opengl_graphics::Texture,
  //wall:          opengl_graphics::Texture,
}

fn load_resources() -> Resources {
  use opengl_graphics::Texture;
  use std::path::Path;

  Resources {
    //big_font:      Texture::from_path(Path::new("images/big-font.png")).unwrap(),
    //floor:         Texture::from_path(Path::new("images/floor.png")).unwrap(),
    //goal_top:      Texture::from_path(Path::new("images/goal-top.png")).unwrap(),
    //goal:          Texture::from_path(Path::new("images/goal.png")).unwrap(),
    //inventory_key: Texture::from_path(Path::new("images/inventory-key.png")).unwrap(),
    //key:           Texture::from_path(Path::new("images/key.png")).unwrap(),
    //locked:        Texture::from_path(Path::new("images/locked.png")).unwrap(),
    player:        Texture::from_path(Path::new("images/player.png")).unwrap(),
    //sign:          Texture::from_path(Path::new("images/sign.png")).unwrap(),
    //small_font:    Texture::from_path(Path::new("images/small-font.png")).unwrap(),
    //spiny:         Texture::from_path(Path::new("images/spiny.png")).unwrap(),
    //start_top:     Texture::from_path(Path::new("images/start-top.png")).unwrap(),
    //start:         Texture::from_path(Path::new("images/start.png")).unwrap(),
    //unlocked_top:  Texture::from_path(Path::new("images/unlocked-top.png")).unwrap(),
    //unlocked:      Texture::from_path(Path::new("images/unlocked.png")).unwrap(),
    //wall:          Texture::from_path(Path::new("images/wall.png")).unwrap(),
  }
}


const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COVER: [f32; 4] = [1.0, 1.0, 1.0, 0.5];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn render(gl: &mut GlGraphics, args: &piston::input::RenderArgs, resources: &Resources, context: &Context, state: &State) {
  use graphics::*;

  let rotation = context.square_rotation;
  let active = state.is_square_activated;
  let (x, y) = ((args.width / 2) as f64,
                (args.height / 2) as f64);

  gl.draw(args.viewport(), |c, gl| {
    // Sharp pixels please!
    unsafe {
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }

    // Clear the screen.
    clear(WHITE, gl);

    let transform = c.transform.trans(x, y)
                               .rot_rad(rotation)
                               .scale(5.0, 5.0)
                               .trans(-5.0, -5.0);

    // Draw the player rotating around the middle of the screen.
    image(&resources.player, transform, gl);

    if active {
      // Cover the whole thing with a semi-transparent white layer
      rectangle(COVER, [0.0, 0.0, args.width as f64, args.height as f64], c.transform, gl);
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
      use RawInputEvent::{ MouseClick };

      match e {
        Render(args)              => render(&mut gl, &args, &resources, &context.sample(), &state.sample()),
        Input(Press(Mouse(Left))) => sink.send(MouseClick),
        _            => ()
      }
    }
}

