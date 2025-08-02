

fn recursive_sum(i: &[u64]) -> u64 {
    println!("{:?}", i);
    // base case:
    if i.len() == 0 {
        return 0;
    } else if i.len() == 1 {
        return i[0];
    } else {
        return i[0] + recursive_sum(&i[1..i.len()]);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_to_sum() {
        assert_eq!(recursive_sum(&[]), 0);
        assert_eq!(recursive_sum(&[1]), 1);
        assert_eq!(recursive_sum(&[1, 2, 3, 4, 5, 6]), 21);
        assert_eq!(recursive_sum(&[10, 10, 10, 10]), 40);
        assert_eq!(recursive_sum(&[10, 9, 8, 7]), 34);
    }
}
