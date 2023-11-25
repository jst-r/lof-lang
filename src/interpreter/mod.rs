mod environment;
mod globals;
mod interpreter_impl;
mod runtime_type;
mod runtime_value;

use self::environment::WrappedEnv;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub environment: WrappedEnv,
}
