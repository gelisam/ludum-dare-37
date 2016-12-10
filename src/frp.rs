extern crate time;

use carboxyl::*;
use carboxyl_time::*;

use types::*;


pub fn frp_network(raw_input_events: &Stream<RawInputEvent>) -> (Signal<Context>, Signal<State>) {
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
