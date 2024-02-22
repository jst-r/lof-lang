use std::rc::Rc;

use super::{
    environment::{EnvironmentTrait, WrappedEnv},
    runtime_type::NativeFunctionWrapper,
    runtime_value::{RuntimeError, RuntimeResult, RuntimeValue},
};

pub fn define_globals(mut env: WrappedEnv) {
    env.define(Rc::from(get_function_name(time)), wrap_native_fn(time));
    env.define(Rc::from(get_function_name(assert)), wrap_native_fn(assert));
}

fn wrap_native_fn<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static>(
    f: F,
) -> RuntimeValue {
    RuntimeValue::Function(Rc::new(NativeFunctionWrapper { function: f }))
}

fn get_function_name<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static>(
    _: F,
) -> &'static str {
    std::any::type_name::<F>().split("::").last().unwrap()
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

fn assert(args: [RuntimeValue; 2]) -> RuntimeResult {
    let RuntimeValue::Bool(condition) = &args[0] else {
        return RuntimeError::TypeError.into();
    };
    let RuntimeValue::String(message) = &args[1] else {
        return RuntimeError::TypeError.into();
    };

    if !condition {
        RuntimeError::AssertionError(message.clone()).into()
    } else {
        RuntimeValue::Unit.into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_function_name() {
        dbg!(super::get_function_name(super::time));
    }
}
