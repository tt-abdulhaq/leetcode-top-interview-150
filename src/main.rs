mod  array_string;

use array_string::{majority_element::majority_element};

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let majority = majority_element(nums);
    println!("The majority element is {}", majority);
}