use num_format::{Locale, ToFormattedString};
use sort::{Sorter, SorterMethod};
use std::sync::{mpsc, Arc};
use std::{env, thread};

/// Create a list of tuples of `Sorter` methods with their associated names.
///
/// These tuples are intended to be used for arguments to `sort::time_sort()`.
macro_rules! sorter_methods {
    ( $( $x:ident ),*, ) => {
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
        insertion_sort,
        merge_sort,
        threaded_merge_sort,
        stalin_sort,
        std_sort,
        std_sort_unstable,
    ];

    let sorter = Arc::new(Sorter::new(length));
    let mut handles = Vec::new();
    let (tx, rx) = mpsc::channel();

    println!(
        "To sort {} items:\n",
        length.to_formatted_string(&Locale::en)
    );

    for (method, name) in sorts {
        let sorter = Arc::clone(&sorter);
        let tx_new = tx.clone();
        handles.push(thread::spawn(move || {
            tx_new
                .send((name, sort::time_sort(&sorter, method)))
                .unwrap();
        }));
    }
    drop(tx);

    for (name, time) in rx {
        println!("{name} took {time:?}");
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
