mod  array_string;

use array_string::merge_sorted_array::merge;

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1);
}