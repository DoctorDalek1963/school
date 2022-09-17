use std::time;

use rand::seq::SliceRandom;
use rand::thread_rng;

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
        let mut temp: u32;

        for _ in 0..(list.len() - 1) {
            for i in 0..iterations {
                if list[i] > list[i + 1] {
                    temp = list[i];
                    list[i] = list[i + 1];
                    list[i + 1] = temp;
                }
            }
            iterations -= 1;
        }

        list
    }

    /// Perform a Stalin sort on the list.
    ///
    /// This works by removing all elements that aren't in order.
    pub fn stalin_sort(&self) -> Vec<u32> {
        let mut list = self.list.clone();
        let mut i: usize = 0;
        let mut highest: u32 = 0;

        while i < list.len() {
            if list[i] < highest {
                list.remove(i);
            } else {
                highest = list[i];
                i += 1;
            }
        }

        list
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
        assert!(is_sorted(&Sorter::new(5).bogo_sort()));
    }

    #[test]
    fn bubble_sort() {
        assert!(is_sorted(&Sorter::new(1000).bubble_sort()));
    }

    #[test]
    fn stalin_sort() {
        assert!(is_sorted(&Sorter::new(1000).stalin_sort()));
    }

    #[test]
    fn std_sort() {
        assert!(is_sorted(&Sorter::new(1000).std_sort()));
    }
}
