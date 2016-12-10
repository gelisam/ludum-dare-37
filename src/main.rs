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


fn render(gl: &mut GlGraphics, args: &piston::input::RenderArgs, context: &Context, state: &State) {
  use graphics::*;

  const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
  const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];

  let image  = graphics::image::Image::new().rect(graphics::rectangle::square(0.0, 0.0, 10.0));
  let texture = opengl_graphics::Texture::from_path(std::path::Path::new("images/player.png")).unwrap();

  let square = rectangle::square(0.0, 0.0, 10.0);
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
    clear(GREEN, gl);

    let transform = c.transform.trans(x, y)
                               .rot_rad(rotation)
                               .scale(5.0, 5.0)
                               .trans(-5.0, -5.0);
    
    // Draw something rotating around the middle of the screen.
    if active {
      image.draw(&texture, &DrawState::default(), transform, gl);
    } else {
      rectangle(RED, square, transform, gl);
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
        Render(args)              => render(&mut gl, &args, &context.sample(), &state.sample()),
        Input(Press(Mouse(Left))) => sink.send(MouseClick),
        _            => ()
      }
    }
}

