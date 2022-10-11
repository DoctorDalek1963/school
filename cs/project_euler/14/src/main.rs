fn get_chain_length(value: u64) -> u64 {
    if value == 1 {
        return 1;
    }

    1 + get_chain_length(match value % 2 {
        0 => value / 2,
        _ => 3 * value + 1,
    })
}

fn main() {
    println!(
        "{:?}",
        (1..1000000)
            .into_iter()
            .map(|v| (v, get_chain_length(v)))
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
    );
}
