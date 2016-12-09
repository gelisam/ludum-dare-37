#[macro_use] extern crate carboxyl;
extern crate carboxyl_time;
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


fn frp_network() -> Signal<Context> {
  let seconds: Signal<f64> = {
    let time: Signal<time::Tm> = now();
    let t0 = time.sample();
    lift!(move |t| (t - t0).num_milliseconds() as f64 / 1000.0, &time)
  };

  let square_rotation = lift!(|t| 2.0 * t, &seconds); // Rotate 2 radians per second.

  lift!(|r|
    Context {
      square_rotation: r,
    },
    &square_rotation
  )
}


fn render(gl: &mut GlGraphics, args: &piston::input::RenderArgs, context: &Context) {
  use graphics::*;

  const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
  const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

  let square = rectangle::square(0.0, 0.0, 50.0);
  let rotation = context.square_rotation;
  let (x, y) = ((args.width / 2) as f64,
                (args.height / 2) as f64);

  gl.draw(args.viewport(), |c, gl| {
    // Clear the screen.
    clear(GREEN, gl);

    let transform = c.transform.trans(x, y)
                               .rot_rad(rotation)
                               .trans(-25.0, -25.0);

    // Draw a box rotating around the middle of the screen.
    rectangle(RED, square, transform, gl);
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

    let context = frp_network();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
      use piston::input::Event::{ Render };

      match e {
        Render(args) => render(&mut gl, &args, &context.sample()),
        _            => ()
      }
    }
}

