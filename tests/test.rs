use lof_lang::interpreter::runtime_value::RuntimeValue;

type UnitResult = Result<(), ()>;

#[test]
fn basic_e2e() -> Result<(), ()> {
    match lof_lang::run_expr("1 + 2") {
        Ok(RuntimeValue::Integer(3)) => Ok(()),
        _ => Err(()),
    }
}
