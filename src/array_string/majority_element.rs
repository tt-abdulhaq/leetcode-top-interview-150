pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut condidate =  0;

    for num in nums{
        if count == 0 {
            condidate = num;
        }
        if condidate == num {
            count += 1;
        }else{
            count -= 1;
        }
    }
    condidate
        
}