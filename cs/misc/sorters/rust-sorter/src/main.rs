use std::env;

mod sort;
use sort::{Sorter, SorterMethod};

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
        //bubble_sort,
        //stalin_sort,
        std_sort
    ];

    let sorter = Sorter::new(length);
    println!("To sort {} items:\n", length);

    // TODO: Parallelize this
    for (method, name) in sorts {
        sort::time_sort(&sorter, method, name)
    }
}
