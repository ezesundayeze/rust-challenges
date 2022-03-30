// A median number is the middle number of a sorted collection
pub fn median_of_an_collection(mut arr: Vec<i32>) -> Option<i32> {
    //sort the collection
    arr.sort();

    //check if the collection is empty
    if arr.is_empty() {
        return None;
    }

    // Check if there are two median numbers, add them and divide them by 2.
    // But first, we'' use modulo to check if the size of the collection is an even number or odd number.
    // If it's an even number, that means we have 2 median numbers, so, we'll quickly add them and dive the result by 2

    if arr.len() % 2 == 0 {
        let left_median = (arr.len() / 2) - 1;
        let right_median = arr.len() / 2;

        let median = (arr[left_median] + arr[right_median]) / 2;
        return Some(median);
    }

    // if there is one median number return it
    return Some(arr[arr.len() / 2]);
}

//We'll use the Rust
pub fn find_and_remove_duplicates<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    arr.sort();
    arr.dedup();
    return arr;
}

//Fibunacci
pub fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}


// Print a generic text
pub fn display_any_text <T: std::fmt::Display>(t: T)-> T{
    return t;
}

//Bubble sort
pub fn bubble<T: Ord>(custom_array: &mut [T]) {
    for n in 0..custom_array.len() {
        for j in 0..custom_array.len() - 1 - n {
            if custom_array[j] > custom_array[j + 1] {
                custom_array.swap(j, j + 1);
            }
        }
    }
}


