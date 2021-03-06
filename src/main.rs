use challenges;
fn main() {

    //Print a generic Text
    let text = challenges::display_any_text("31231");
    //Find and Remove duplicates
    let unique = challenges::find_and_remove_duplicates(vec![3,3,45,6,8,4,4,4]);

    //Fibunacci sequence
    let fib = challenges::fib(20);

    //Bubble sort
    let mut custom_collection =  vec![1,3,5,20,4,80,5,8];
    let bubble_sort = challenges::bubble(&mut custom_collection);

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

    //Test print generic text
    assert_eq!(text, "31231");

    // Test Bubble sort
    assert_eq!(bubble_sort, vec![1,3,4,5,5,8,20,80] )
}
