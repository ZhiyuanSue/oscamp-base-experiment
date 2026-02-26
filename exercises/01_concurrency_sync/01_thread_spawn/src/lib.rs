//! # Thread Creation
//!
//! In this exercise, you will learn how to create threads and pass data between threads.
//!
//! ## Concepts
//! - `std::thread::spawn` creates a new thread
//! - `move` closures capture variable ownership
//! - `JoinHandle::join()` waits for thread completion and retrieves return value

use std::thread;

/// Multiply each element of a vector by 2 in a new thread, returning the result vector.
///
/// Hint: Use `thread::spawn` and `move` closure.
pub fn double_in_thread(numbers: Vec<i32>) -> Vec<i32> {
    // TODO: Create a new thread to multiply each element of numbers by 2
    // Use thread::spawn and move closure
    // Use join().unwrap() to get result
    todo!()
}

/// Sum two vectors in parallel, returning a tuple of two sums.
///
/// Hint: Create two threads for each vector.
pub fn parallel_sum(a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    // TODO: Create two threads to sum a and b respectively
    // Join both threads to get results
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_basic() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(double_in_thread(nums), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_double_empty() {
        assert_eq!(double_in_thread(vec![]), vec![]);
    }

    #[test]
    fn test_double_negative() {
        assert_eq!(double_in_thread(vec![-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_parallel_sum() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20, 30];
        assert_eq!(parallel_sum(a, b), (6, 60));
    }

    #[test]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![], vec![]), (0, 0));
    }
}
