use std::env;

mod sort;
use sort::{Sorter, SorterMethod};

/// Run the various sorts and time them.
fn main() {
    let default_length: u32 = 1000;
    let length: u32 = match env::args().collect::<Vec<String>>().get(1) {
        Some(num) => num.parse().unwrap_or(default_length),
        None => default_length,
    };

    // TODO: Make this a macro eventually
    let sorts = [
        //(Sorter::bogo_sort as SorterMethod, "bogo_sort"),
        (Sorter::bubble_sort as SorterMethod, "bubble_sort"),
        (Sorter::std_sort as SorterMethod, "std_sort"),
    ];

    let sorter = Sorter::new(length);
    println!("To sort {} items:\n", length);
    for (method, name) in sorts {
        sort::time_sort(&sorter, method, name)
    }
}
