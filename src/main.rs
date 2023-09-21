use rand::prelude::*;
use std::io::stdin;
use std::time::Instant;

fn main() {
    const REPEATS: usize = 11;
    let mut bf_log = [0.0;REPEATS];
    let mut dc_log = [0.0;REPEATS];
    let mut kd_log = [0.0;REPEATS];

    println!("Provide size of an array. Array will be populated by random numbers from -1000 to 1000 (inclusive)");    
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

    println!("RUN - Brute Force - D&C - Kadane");
    for i in 0..REPEATS{
        let mut nums = vec![];
        for _ in 0..length {
            let random = rand::thread_rng().gen_range(-1000..=1000);
            nums.push(random);
        }

        let mut timer;
        let bf_result;
        let dc_result;
        let kd_result;
        let mut row_string = format!("{}\t", i+1); 

        timer = Instant::now();
        bf_result = brute_force(&nums);
        bf_log[i] = timer.elapsed().as_secs_f64() * 1_000_000.0; //micro sec
        row_string.push_str(format!("{:.2}\t", bf_log[i]).as_str());

        timer = Instant::now();
        dc_result = divide_and_conquer(&nums, 0, nums.len() - 1 as usize);
        dc_log[i] = timer.elapsed().as_secs_f64() * 1_000_000.0; //micro sec
        row_string.push_str(format!("{:.2}\t", dc_log[i]).as_str());

        timer = Instant::now();
        kd_result = kadane(&nums); 
        kd_log[i] = timer.elapsed().as_secs_f64() * 1_000_000.0; //micro sec 
        row_string.push_str(format!("{:.2}\t", kd_log[i]).as_str());
        println!("{}", row_string);

        if !(bf_result == dc_result && bf_result == kd_result){
            panic!();
        }
    }
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
