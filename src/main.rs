fn main() {
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 2 {
        return 1
    }
    fibonacci(n-1) + fibonacci(n-2)
}

