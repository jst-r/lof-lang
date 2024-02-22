const SOURCE: &str = r#"
class Test {}

var a = Test();
a.b = 1;
a.b = a.b + 1;

print(a.b);

assert(a.b == 2, "");
assert(a.b != 2, "This should fail");
"#;

// print "fib test";
// fn fib(n) {
//     if n <= 2 {
//         1
//     } else {
//         fib(n - 1) + fib(n - 2)
//     }
// }
// print fib(25);

fn main() {
    lof_lang::run_code(SOURCE);
    // dbg!(lof_lang::parse_expression("1 + 2"));
    // dbg!(lof_lang::run_expr("1+2"));
}
