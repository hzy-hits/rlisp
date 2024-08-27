pub mod evaluator;
pub mod myparser;
pub mod types;

pub use evaluator::eval;
pub use myparser::Parser;
pub use types::{environment::default_env, environment::RispEnv, errors::RispError, expr};
