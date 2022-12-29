use std::collections::HashMap;

fn main() {
    let mut nums = [5, 4, 4, 3, 2, 1, 0];
    nums.sort();

    println!("The median of nums is: {}", find_median(&nums));
    println!("The mode of nums is: {}", find_mode(&nums));
}

fn find_median(nums: &[i32]) -> i32 {
    nums[nums.len() / 2]
}

fn find_mode(nums: &[i32]) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    *map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot find the mode of zero numbers.")
}
