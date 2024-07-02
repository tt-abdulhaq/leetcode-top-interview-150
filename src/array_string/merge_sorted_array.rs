

pub fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &Vec<i32>, n: usize) {
    let (mut i, mut j, mut k) = (m as isize - 1, n as isize - 1, (m + n) as isize - 1);

    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }

    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        k -= 1;
        j -= 1;
    }
}
