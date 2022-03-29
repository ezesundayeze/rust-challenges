pub fn median_of_an_collection(mut arr: Vec<i32>) -> Option<i32> {
    //sort the collection
    arr.sort();

    //check if the collection is empty
    if arr.is_empty() {
        return None;
    }

    // Check if there are two median numbers, add them and divide them by 2
    if arr.len() % 2 == 0 {
        let left_median = (arr.len() / 2) - 1;
        let right_median = arr.len() / 2;

        let median = (arr[left_median] + arr[right_median]) / 2;
        return Some(median);
    }
    return Some(arr[arr.len() / 2]);
}

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
