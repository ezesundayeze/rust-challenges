use challenges;
fn main() {

    //Find and Remove duplicates
    let unique = challenges::find_and_remove_duplicates(vec![3,3,45,6,8,4,4,4]);

    //Fibunacci sequence
    let fib = challenges::fib(20);

    // Median
    let median = challenges::median_of_an_collection(vec![1,45,7,89,97,3]);
    let median2 = challenges::median_of_an_collection(vec![1,45,7,89,97,7,3]);
    let median3 = challenges::median_of_an_collection(vec![]);
    
    // Tests
    assert_eq!(median, Some(26));
    assert_eq!(median2, Some(7));
    assert_eq!(median3, None);

    // test fib
    assert_eq!(fib, 6765);

    //Test remove duplicate
    assert_eq!(unique, vec![3,4,6,8,45]);
}
