use std::rc::Rc;

use super::runtime::{
    function::NativeFunctionWrapper,
    result::{RuntimeError, RuntimeResult},
    value::RuntimeValue,
};

pub fn wrap_native_fn<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static>(
    f: F,
) -> RuntimeValue {
    let function = RuntimeValue::Function(Rc::new(NativeFunctionWrapper { function: f }));
    function
}

pub fn get_function_name<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static>(
    _: F,
) -> &'static str {
    std::any::type_name::<F>().split("::").last().unwrap()
}

pub fn time(_: [RuntimeValue; 0]) -> RuntimeResult {
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

pub fn assert(args: [RuntimeValue; 2]) -> RuntimeResult {
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
        assert_eq!(super::get_function_name(super::time), "time");
    }
}
