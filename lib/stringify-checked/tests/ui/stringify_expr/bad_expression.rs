use stringify_checked::stringify_expr;

fn main() {
    stringify_expr!(match Some(0) {});
}
