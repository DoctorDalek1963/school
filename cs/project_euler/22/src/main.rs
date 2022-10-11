use std::collections::HashMap;

mod names;

fn get_alpha_score(name: &str) -> u64 {
    let alpha_score_map: HashMap<char, u64> = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('D', 4),
        ('E', 5),
        ('F', 6),
        ('G', 7),
        ('H', 8),
        ('I', 9),
        ('J', 10),
        ('K', 11),
        ('L', 12),
        ('M', 13),
        ('N', 14),
        ('O', 15),
        ('P', 16),
        ('Q', 17),
        ('R', 18),
        ('S', 19),
        ('T', 20),
        ('U', 21),
        ('V', 22),
        ('W', 23),
        ('X', 24),
        ('Y', 25),
        ('Z', 26),
    ]);

    // All the input characters are guaranteed to be uppercase Latin letters
    name.chars().map(|c| alpha_score_map.get(&c).unwrap()).sum()
}

fn main() {
    let sorted_names = {
        let mut names = names::NAMES_UNSORTED;
        names.sort();
        names
    };

    let score_sum: u64 = sorted_names
        .iter()
        .enumerate()
        .map(|(i, name)| ((i + 1) as u64) * get_alpha_score(name))
        .sum();

    println!("{:?}", score_sum);
}
