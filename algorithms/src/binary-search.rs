fn bin_search(arr: &[i64], a: i64) -> Result<usize, ()> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid: usize = (left + right) / 2;
        if a == arr[mid] {
            return Ok(mid);
        } else if a > arr[mid] {
            left = mid + 1;
        } else if a < arr[mid] {
            if mid == 0 {
                return Err(());
            }
            right = mid - 1;
        }
    }
    Err(())
}

fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 7, 8, 10, 11, 12, 15];
    match bin_search(&v, 7) {
        Ok(x) => println!("{}", x),
        Err(()) => println!("Err: Total piss."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn off_left() {
        let v = vec![-23, 1, 2, 3, 6, 10, 11, 12, 13, 83, 91, 100];
        assert_eq!(bin_search(v.as_slice(), -99), Err(()));
        assert_eq!(bin_search(v.as_slice(), -24), Err(()));
    }

    #[test]
    fn off_right() {
        let v = vec![-23, 1, 2, 3, 6, 10, 11, 12, 13, 83, 91, 100];
        assert_eq!(bin_search(v.as_slice(), 101), Err(()));
        assert_eq!(bin_search(v.as_slice(), 10000), Err(()));
    }

    #[test]
    fn missing_in_range() {
        let v = vec![-23, 1, 2, 3, 6, 10, 11, 12, 13, 83, 91, 100];
        assert_eq!(bin_search(v.as_slice(), 0), Err(()));
        assert_eq!(bin_search(v.as_slice(), 9), Err(()));
        assert_eq!(bin_search(v.as_slice(), -22), Err(()));
        assert_eq!(bin_search(v.as_slice(), 99), Err(()));
    }

    #[test]
    fn good_searches() {
        let v = vec![-23, 1, 2, 3, 6, 10, 11, 12, 13, 83, 91, 100];
        assert_eq!(bin_search(v.as_slice(), v[0]), Ok(0));
        assert_eq!(bin_search(v.as_slice(), v[v.len() - 1]), Ok(v.len() - 1));
        assert_eq!(bin_search(v.as_slice(), v[v.len() / 2]), Ok(v.len() / 2));
        assert_eq!(bin_search(v.as_slice(), v[0]), Ok(0));
    }
}
