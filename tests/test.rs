use lof_lang::{interpreter::runtime::value::RuntimeValue, LofError};

#[test]
fn basic_e2e() -> Result<(), ()> {
    match lof_lang::run_expr("1 + 2") {
        Ok(RuntimeValue::Integer(3)) => Ok(()),
        _ => Err(()),
    }
}

#[test]
fn class() -> Result<(), LofError> {
    lof_lang::run_code(include_str!("lof_native/class.lof"))
}

#[test]
fn fibonacci() -> Result<(), LofError> {
    lof_lang::run_code(include_str!("lof_native/fibonacci.lof"))
}

#[test]
fn comment() -> Result<(), LofError> {
    lof_lang::run_code(r#"// This is a comment"#)
}
