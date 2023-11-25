mod environment;
mod interpreter_impl;
mod runtime_type;
mod runtime_value;

use self::environment::Environment;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub environment: Environment,
}
