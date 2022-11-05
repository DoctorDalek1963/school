fn count_divisors(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        divisors::get_divisors(n).len() + 2
    }
}

fn get_nth_triangle_number(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn main() {
    println!(
        "{}",
        (1usize..)
            .find_map(|n| {
                let triangle = get_nth_triangle_number(n);
                if count_divisors(triangle) > 500 {
                    Some(triangle)
                } else {
                    None
                }
            })
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_divisors_test() {
        let divisor_map = vec![(1, 1), (3, 2), (6, 4), (10, 4), (15, 4), (21, 4), (28, 6)];

        for (n, count) in divisor_map {
            assert_eq!(count_divisors(n), count, "n = {n}; count = {count}");
        }
    }

    #[test]
    fn get_nth_triangle_number_test() {
        for i in 1..1000 {
            assert_eq!(get_nth_triangle_number(i), (1..=i).sum::<usize>());
        }
    }
}
