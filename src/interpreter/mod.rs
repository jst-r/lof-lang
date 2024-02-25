pub mod builtins;
pub mod environment;
pub mod globals;
pub mod interpreter_impl;
pub mod resolver;
pub mod runtime_type;
pub mod runtime_value;

use std::collections::BTreeMap;

use self::environment::WrappedEnv;

#[derive(Debug, Default)]
pub struct Interpreter {
    pub environment: WrappedEnv,
    pub locals: BTreeMap<usize, usize>,
    pub globals: WrappedEnv,
}
