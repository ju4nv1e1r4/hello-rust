fn mover_zeros(nums: &mut Vec<i32>) {
    let mut non_zero_index = 0;

    for i in 0..nums.len(){
        if nums[i] != 0 {
            nums.swap(i, non_zero_index);
            non_zero_index += 1;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 2, 0, 3, 5, 0, 6];

    mover_zeros(&mut nums);

    println!("{:?}", nums);
}
