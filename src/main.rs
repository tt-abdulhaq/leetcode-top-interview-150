mod  array_string;

use array_string::jump_game::can_jump;

fn main() {
    let mut nums = vec![3,2,1,0,4];
    println!("{:?}", nums);
    let a = can_jump(nums);
    println!("{}-->", a);
    
}