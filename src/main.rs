mod  array_string;

use array_string::{majority_element::majority_element, rotate_array::rotate};

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", nums);
    let k = 3;
    rotate(&mut nums, k);
    
}