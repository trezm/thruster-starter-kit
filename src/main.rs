extern crate thruster;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate futures;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

mod context;
mod connectivity;

use context::{generate_context, Ctx};
use connectivity::{init as connectivity_routes};
use thruster::{App, MiddlewareChain, MiddlewareReturnValue};
use time::Duration;
use std::boxed::Box;
use futures::{future, Future};

lazy_static! {
  static ref APP: App<Ctx> = {
    let mut _app = App::<Ctx>::create(generate_context);

    _app.use_middleware("/", profiling);

    // Add a subroute, this functions like its own app within the main app.
    _app.use_sub_app("/ping", &connectivity_routes());

    _app.get("/json", vec![json]);
    _app.get("/plaintext", vec![plaintext]);

    _app.set404(vec![not_found_404]);

    _app
  };
}

fn not_found_404(context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let mut context = Ctx::new(context);

  context.body = "<html>
  ( ͡° ͜ʖ ͡°) What're you looking for here?
</html>".to_owned();
  context.set_header("Content-Type".to_owned(), "text/html".to_owned());
  context.status_code = 404;

  Box::new(future::ok(context))
}

fn profiling(context: Ctx, chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let start_time = time::now();

  let ctx_future = chain.next(context)
      .and_then(move |ctx| {
        let elapsed_time: Duration = time::now() - start_time;
        println!("[{}μs] {} -- {}",
          elapsed_time.num_microseconds().unwrap(),
          ctx.method.clone(),
          ctx.path.clone());

        future::ok(ctx)
      });

  Box::new(ctx_future)
}

#[derive(Serialize)]
struct JsonStruct<'a> {
  message: &'a str
}
fn json(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let json = JsonStruct {
    message: "Hello, World!"
  };

  let val = serde_json::to_string(&json).unwrap();

  context.body = val;
  context.set_header("Server".to_owned(), "thruster".to_owned());
  context.set_header("Content-Type".to_owned(), "application/json".to_owned());

  Box::new(future::ok(context))
}

fn plaintext(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  let val = "Hello, World!".to_owned();

  context.body = val;
  context.set_header("Server".to_owned(), "thruster".to_owned());
  context.set_header("Content-Type".to_owned(), "text/plain".to_owned());

  Box::new(future::ok(context))
}

fn main() {
  println!("Starting server...");

  App::start(&APP, "0.0.0.0".to_string(), "8080".to_string());
}
