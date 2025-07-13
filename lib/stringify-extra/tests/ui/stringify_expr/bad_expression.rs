use stringify_extra::stringify_expr;

fn main() {
    stringify_expr!(match Some(0) {});
}
