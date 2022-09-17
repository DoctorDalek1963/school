use sort::{Sorter, SorterMethod};
use std::env;
use threadpool::ThreadPool;

mod sort;

/// Create a list of tuples of `Sorter` methods with their associated names.
///
/// These tuples are intended to be used for arguments to `sort::time_sort()`.
macro_rules! sorter_methods {
    ( $( $x:ident ),* ) => {
        {
            let mut temp_vec: Vec<(SorterMethod, &str)> = Vec::new();
            $(
                temp_vec.push((Sorter::$x as SorterMethod, stringify!($x)));
            )*
            temp_vec
        }
    };
}

/// Run the various sorts and time them.
fn main() {
    let default_length: u32 = 1000;
    let length: u32 = match env::args().collect::<Vec<String>>().get(1) {
        Some(num) => num.parse().unwrap_or(default_length),
        None => default_length,
    };

    let sorts = sorter_methods![
        //bogo_sort,
        bubble_sort,
        merge_sort,
        stalin_sort,
        std_sort
    ];

    let sorter = Sorter::new(length);
    println!("To sort {} items:\n", length);

    let pool = ThreadPool::new(sorts.len());
    for (method, name) in sorts {
        let clone = sorter.clone();
        pool.execute(move || sort::time_sort(&clone, method, name));
    }
    pool.join();
}
