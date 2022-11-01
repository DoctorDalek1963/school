fn get_proper_divisors(n: u16) -> Vec<u16> {
    (1..=(n / 2)).filter(|x| n % x == 0).collect()
}

fn is_amicable(n: u16) -> bool {
    let s = get_proper_divisors(n).iter().sum();
    s != n && get_proper_divisors(s).iter().sum::<u16>() == n
}

fn main() {
    let amicable_numbers: Vec<_> = (1..=10_000).filter(|&n| is_amicable(n)).collect();
    println!("amicable_numbers = {:#?}", amicable_numbers);
    println!("sum = {}", amicable_numbers.iter().sum::<u16>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_proper_divisors_test() {
        assert_eq!(get_proper_divisors(6), vec![1, 2, 3]);
        assert_eq!(get_proper_divisors(24), vec![1, 2, 3, 4, 6, 8, 12]);
        assert_eq!(
            get_proper_divisors(220),
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]
        );
        assert_eq!(get_proper_divisors(284), vec![1, 2, 4, 71, 142]);
    }

    #[test]
    fn is_amicable_test() {
        assert!(is_amicable(220));
        assert!(is_amicable(284));

        assert!(!is_amicable(16));
        assert!(!is_amicable(912));
        assert!(!is_amicable(1107));
    }
}
