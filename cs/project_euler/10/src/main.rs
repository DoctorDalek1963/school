use primes::{PrimeSet, Sieve};

fn main() {
    let mut pset = Sieve::new();
    println!(
        "{:?}",
        pset.iter().take_while(|v| v < &2_000_000u64).sum::<u64>()
    );
}
