use std::rc::Rc;

use super::{
    environment::{EnvironmentTrait, WrappedEnv},
    runtime_type::NativeFunctionWrapper,
    runtime_value::{RuntimeResult, RuntimeValue},
};

pub fn define_globals(mut env: WrappedEnv) {
    env.define(Rc::from("time"), wrap_native_fn(time));
}

fn wrap_native_fn<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static>(
    f: F,
) -> RuntimeValue {
    RuntimeValue::Function(Rc::new(NativeFunctionWrapper { function: f }))
}

fn time(_: [RuntimeValue; 0]) -> RuntimeResult {
    use std::time::{SystemTime, UNIX_EPOCH};
    RuntimeValue::Integer(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap(),
    )
    .into()
}
