use std::env;

fn factorial(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(1);
    } else {
        match factorial(n - 1) {
            None => return None,
            Some(x) => return n.checked_mul(x),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need one arg.")
    }
    let n: u64 = args[args.len() - 1].parse().unwrap();
    println!("Hello, world!");
    match factorial(n) {
        Some(x) => println!("Factorial {} = {}", n, x),
        None => println!("Oh what! Overflow."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lowish_values() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
        assert_eq!(factorial(2), Some(2));
        assert_eq!(factorial(3), Some(3 * 2));
        assert_eq!(factorial(5), Some(5 * 4 * 3 * 2));
    }

    #[test]
    fn big_number() {
        assert_eq!(factorial(17), Some(355687428096000));
    }

    #[test]
    fn none_on_overflow() {
        assert_eq!(factorial(100), None);
    }
}
