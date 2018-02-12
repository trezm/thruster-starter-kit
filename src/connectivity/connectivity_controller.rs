use context::{Ctx};
use fanta::{MiddlewareChain};

pub fn ping(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  context.body = "pong".to_string();

  context
}
