use rand::prelude::*;
use std::io::stdin;
use std::time::Instant;

fn main() {
    println!("Provide size of an array. Array will be populated by random number from -1000 to 1000 inclusive");    
    let length: u32;
    loop{
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).expect("Error reading console input");
        match input_str.trim().parse::<u32>(){
            Ok(input) => {
                length = input; 
                break;
            }
            Err(_) =>{
                println!("Input is incorrect. Please provide unassigned 32 bit integer.");
            }
        }
    }

    let mut nums = vec![];
    for _ in 0..length {
        let random = rand::thread_rng().gen_range(-1000..=1000);
        nums.push(random);
    }

    let start = Instant::now();
    let bf_result = brute_force(&nums);
    println!(
        "RESULT: {bf_result}; TIME_IN_MICROS: {}",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    let dac_result = divide_and_conquer(&nums, 0, nums.len() - 1 as usize);
    println!(
        "RESULT: {dac_result}; TIME_IN_MICROS: {}",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    let kadane_result = kadane(&nums); 
    println!(
        "RESULT: {kadane_result}; TIME_IN_MICROS: {}", start.elapsed().as_micros()
    );
}

fn brute_force(nums: &Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..nums.len() {
        let mut local_sum = 0;
        for j in i..nums.len() {
            local_sum += nums[j];
            max_sum = if local_sum > max_sum { local_sum } else { max_sum };
        }
    }

    max_sum
}

fn divide_and_conquer(nums: &Vec<i32>, low: usize, high: usize) -> i32 {
    if high == low {
        return nums[low];
    } else {
        let mid = (low + high) / 2;
        let left_sum = divide_and_conquer(nums, low, mid);
        let right_sum = divide_and_conquer(nums, mid + 1, high);
        let cross_sum = divide_and_conquer_crossing(nums, low, mid, high);

        left_sum.max(right_sum).max(cross_sum)
    }
}

fn divide_and_conquer_crossing(nums: &Vec<i32>, low: usize, mid: usize, high: usize) -> i32 {
    let mut left_sum = std::i32::MIN;

    let mut sum = 0;
    for i in (low..=mid).rev() {
        sum = sum + nums[i];
        left_sum = if sum > left_sum { sum } else { left_sum };
    }

    let mut right_sum = std::i32::MIN;
    sum = 0;
    for j in mid + 1..=high {
        sum = sum + nums[j];
        right_sum = if sum > right_sum { sum } else { right_sum };
    }

    left_sum + right_sum
}


fn kadane(nums: &Vec<i32>) -> i32{
    let mut max_current = i32::MIN;
    let mut max_cumulative = 0;

    for num in nums{
        max_cumulative += *num;
        max_current = if max_cumulative > max_current { max_cumulative } else { max_current };
        max_cumulative = if max_cumulative < 0 { 0 } else { max_cumulative };
    }

    max_current
}
