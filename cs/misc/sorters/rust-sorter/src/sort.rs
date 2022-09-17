use std::time;

use rand::seq::SliceRandom;
use rand::thread_rng;

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

    /// Perform a merge sort on the list.
    pub fn merge_sort(&self) -> Vec<u32> {
        fn recursive_merge_sort(list: &mut [u32]) {
            if list.len() < 2 {
                return;
            }

            let mid = list.len() / 2;

            // Sort the left and right halves individually
            recursive_merge_sort(&mut list[..mid]);
            recursive_merge_sort(&mut list[mid..]);

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
            for i in 0..list.len() {
                list[i] = vec[i];
            }
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
        let mut indices: Vec<usize> = Vec::new();

        // Get all the indices of elements we want to keep
        for i in 0..list.len() {
            if list[i] > highest {
                highest = list[i];
                indices.push(i);
            }
        }

        // Then create a new vec of just the elements we care about
        let mut new_list: Vec<u32> = Vec::new();
        for i in indices {
            new_list.push(list[i]);
        }
        new_list
    }

    /// Sort the list with the standard library `Vec::sort` method.
    pub fn std_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        list.sort();
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

pub fn time_sort(sorter: &Sorter, method: SorterMethod, name: &str) {
    let start = time::Instant::now();
    method(sorter);
    let end = time::Instant::now();

    println!("{name} took {:?}", end.duration_since(start));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the given `Sorter` method with the optionally given list length.
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
}
