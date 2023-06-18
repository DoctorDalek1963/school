use num_format::{Locale, ToFormattedString};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{self, Duration};

#[derive(Clone)]
pub struct Sorter {
    list: Vec<u32>,
}

impl Sorter {
    #![allow(dead_code)]

    /// Create a new list with `length` elements and shuffle it.
    pub fn new(length: u32) -> Self {
        let mut list: Vec<u32> = (0..length).collect();
        list.shuffle(&mut thread_rng());
        Self { list }
    }

    /// Perform a bogo sort on the list.
    ///
    /// WARNING: Bogo sort shuffles the whole list until it's sorted.
    /// Anything more than a few elements will take a long time.
    pub fn bogo_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        let mut rng = thread_rng();

        while !is_sorted(&list) {
            list.shuffle(&mut rng);
        }
        list
    }

    /// Perform a bubble sort on the list.
    pub fn bubble_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        let mut iterations = list.len() - 1;

        for _ in 0..(list.len() - 1) {
            for i in 0..iterations {
                if list[i] > list[i + 1] {
                    (list[i], list[i + 1]) = (list[i + 1], list[i]);
                }
            }
            iterations -= 1;
        }

        list
    }

    /// Perform an insertion sort on the list.
    pub fn insertion_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();

        for j in 1..list.len() {
            let next_item = list[j];
            let mut i = j - 1;

            let mut index: usize = i + 1;

            while list[i] > next_item {
                list[i + 1] = list[i];
                index = i;
                if i == 0 {
                    break;
                };
                i -= 1;
            }

            list[index] = next_item;
        }

        list
    }

    /// Perform a merge sort on the list.
    pub fn merge_sort(&self) -> Vec<u32> {
        fn recursive_merge_sort(list: &mut [u32]) {
            if list.len() < 2 {
                return;
            }

            let mid = list.len() / 2;
            let (left, right) = list.split_at_mut(mid);

            // Sort the left and right halves individually
            recursive_merge_sort(left);
            recursive_merge_sort(right);

            // Now we create a vector to store the newly merged list and scan through each half,
            // adding the smaller number each iteration
            let mut left_index = 0;
            let mut right_index = mid;
            let mut vec = Vec::with_capacity(list.len());

            while left_index < mid && right_index < list.len() {
                if list[left_index] < list[right_index] {
                    vec.push(list[left_index]);
                    left_index += 1;
                } else {
                    vec.push(list[right_index]);
                    right_index += 1;
                }
            }

            // One of these slices will be empty, but the other will contain the unmerged, sorted
            // elements
            for elem in &list[left_index..mid] {
                vec.push(*elem);
            }
            for elem in &list[right_index..] {
                vec.push(*elem);
            }

            // Then we just put the elements back into the list slice
            list.copy_from_slice(&vec[..list.len()]);
        }

        let mut list = self.list.clone();
        recursive_merge_sort(&mut list[..]);
        list
    }

    /// Perform a multi-threaded merge sort on the list.
    ///
    /// See [`Self::merge_sort`].
    pub fn threaded_merge_sort(&self) -> Vec<u32> {
        const THRESHOLD: usize = 100_000;

        fn recursive_merge_sort(list: &mut [u32]) {
            if list.len() < 2 {
                return;
            }

            let mid = list.len() / 2;
            let (left, right) = list.split_at_mut(mid);

            // It's expensive to always spawn new threads, so only do it if the length is above a
            // certain THRESHOLD.
            if left.len() > THRESHOLD || right.len() > THRESHOLD {
                use std::thread;

                thread::scope(|s| {
                    s.spawn(|| recursive_merge_sort(left));
                    s.spawn(|| recursive_merge_sort(right));
                });
            } else {
                recursive_merge_sort(left);
                recursive_merge_sort(right);
            }

            // Now we create a vector to store the newly merged list and scan through each half,
            // adding the smaller number each iteration
            let mut left_index = 0;
            let mut right_index = mid;
            let mut vec = Vec::with_capacity(list.len());

            while left_index < mid && right_index < list.len() {
                if list[left_index] < list[right_index] {
                    vec.push(list[left_index]);
                    left_index += 1;
                } else {
                    vec.push(list[right_index]);
                    right_index += 1;
                }
            }

            // One of these slices will be empty, but the other will contain the unmerged, sorted
            // elements
            for elem in &list[left_index..mid] {
                vec.push(*elem);
            }
            for elem in &list[right_index..] {
                vec.push(*elem);
            }

            // Then we just put the elements back into the list slice
            list.copy_from_slice(&vec[..list.len()]);
        }

        if self.list.len() <= 2 * THRESHOLD {
            eprintln!(
                "WARNING: threaded_merge_sort is only an advantage over merge_sort for \
                lists with more than {} items",
                (2 * THRESHOLD).to_formatted_string(&Locale::en)
            );
        }

        let mut list = self.list.clone();
        recursive_merge_sort(&mut list[..]);
        list
    }

    /// Perform a Stalin sort on the list.
    ///
    /// This works by removing all elements that aren't in order.
    pub fn stalin_sort(&self) -> Vec<u32> {
        let list = self.list.clone();
        let mut highest: u32 = 0;
        let mut new_list: Vec<u32> = Vec::new();

        // Just take the elements that we want to keep
        for item in list {
            if item > highest {
                highest = item;
                new_list.push(item);
            }
        }
        new_list
    }

    /// Sort the list with the standard library `Vec::sort` method.
    pub fn std_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        list.sort();
        list
    }

    /// Sort the list with the standard library `Vec::sort_unstable` method.
    pub fn std_sort_unstable(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        list.sort_unstable();
        list
    }
}

/// Check if the given list is sorted in ascending order.
fn is_sorted(list: &Vec<u32>) -> bool {
    for i in 0..(list.len() - 1) {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

pub type SorterMethod = fn(&Sorter) -> Vec<u32>;

/// Time the given `Sorter` method with the given sorter and printable name.
pub fn time_sort(sorter: &Sorter, method: SorterMethod) -> Duration {
    let start = time::Instant::now();
    method(sorter);
    let end = time::Instant::now();

    end.duration_since(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the given `Sorter` method with the given list length.
    macro_rules! test_sorter_method {
        ( $meth:ident, $x:literal ) => {{
            assert!(is_sorted(&Sorter::new($x).$meth()));
        }};
    }

    /// Run the given body `x` number of times to ensure good tests.
    macro_rules! test_multiple {
        ( $x:literal, $body:expr ) => {{
            for _ in 0..$x {
                $body;
            }
        }};
    }

    #[test]
    fn sorter_new() {
        assert_eq!(Sorter::new(10).list.len(), 10);
        assert_eq!(Sorter::new(100).list.len(), 100);
        assert_eq!(Sorter::new(1000).list.len(), 1000);
        assert_eq!(Sorter::new(10_000).list.len(), 10_000);
        assert_eq!(Sorter::new(100_000).list.len(), 100_000);
    }

    #[test]
    fn bogo_sort() {
        test_multiple!(100, test_sorter_method!(bogo_sort, 5));
    }

    #[test]
    fn bubble_sort() {
        test_multiple!(10, test_sorter_method!(bubble_sort, 1000));
    }

    #[test]
    fn insertion_sort() {
        test_multiple!(100, test_sorter_method!(insertion_sort, 1000));
    }

    #[test]
    fn merge_sort() {
        test_multiple!(100, test_sorter_method!(merge_sort, 10_000));
    }

    #[test]
    fn stalin_sort() {
        test_multiple!(100, test_sorter_method!(stalin_sort, 10_000));
    }

    #[test]
    fn std_sort() {
        test_multiple!(100, test_sorter_method!(std_sort, 10_000));
    }

    #[test]
    fn std_sort_unstable() {
        test_multiple!(100, test_sorter_method!(std_sort_unstable, 10_000));
    }
}
