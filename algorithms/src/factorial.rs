use std::env;

//fn factorial(n: u64) -> Result<u64, ()> {
fn factorial(n: u64) -> u64 {
   if n<=1 { return 1 }
   else { return n * factorial(n-1) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("Oh fuck")}
    let n: u64 = args[args.len()-1].parse().unwrap();
    println!("Hello, world!");
    println!("Factorial {} = {}", n, factorial(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lowish_values() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 3*2);
        assert_eq!(factorial(5), 5*4*3*2);
    }

    #[test]
    fn big_number() {
        assert_eq!(factorial(17), 355687428096000);

    }

}
