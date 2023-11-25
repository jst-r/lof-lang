pub mod environment;
pub mod globals;
pub mod interpreter_impl;
pub mod runtime_type;
pub mod runtime_value;

use self::environment::WrappedEnv;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub environment: WrappedEnv,
}
