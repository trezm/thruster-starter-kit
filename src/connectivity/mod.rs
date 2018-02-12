mod connectivity_controller;

use fanta::{App};
use context::{generate_context, Ctx};
use self::connectivity_controller::ping;

pub fn init() -> App<Ctx> {
  let mut app = App::<Ctx>::create(generate_context);

  app.get("/", vec![ping]);

  app
}
