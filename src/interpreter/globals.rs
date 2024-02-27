use std::rc::Rc;

use super::{
    builtins::{assert, get_function_name, time, wrap_native_fn},
    environment::{EnvironmentTrait, WrappedEnv},
    runtime::{result::RuntimeResult, value::RuntimeValue},
};

pub fn define_globals(mut env: WrappedEnv) {
    wrap_and_define_native_function(&mut env, time);
    wrap_and_define_native_function(&mut env, assert);
}

fn wrap_and_define_native_function<
    const N: usize,
    F: Fn([RuntimeValue; N]) -> RuntimeResult + 'static + Clone,
>(
    env: &mut WrappedEnv,
    f: F,
) {
    env.define(Rc::from(get_function_name(f.clone())), wrap_native_fn(f));
}
