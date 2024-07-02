pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut j = 0;

    for i in 1..nums.len() {
        if nums[i] != nums[j] {
            j += 1;
            nums[j] = nums[i];
        }
    }

    (j + 1) as i32
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut j = 0;

    for i in 1..nums.len()-1 {
        if nums[i] != nums[j] {
            j += 1;
            nums[j] = nums[i];
        }
    }

    (j + 1) as i32
}
