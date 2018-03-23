use context::{Ctx};
use thruster::{MiddlewareChain, MiddlewareReturnValue};
use std::boxed::Box;
use futures::future;

pub fn ping(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
  context.body = "pong".to_string();

  Box::new(future::ok(context))
}
