use std::collections::HashMap;

fn main() {
    let mut nums = vec![2, 3, 9, 1, 8, 2];
    let mut sum = 0.0;

    for n in &nums {
        sum += *n as f32;
    }

    let mut count_map = HashMap::new();

    for n in &nums {
        let count = count_map.entry(n).or_insert(0);
        *count += 1;
    }

    println!("{:?}", count_map);


    nums.sort();
    let mean = sum / (nums.len() as f32);
    let median = nums[nums.len() / 2];
    println!("sum:{}", sum);
    println!("mean:{}", mean);
    println!("median:{}", median);
    println!("{:?}", nums);
}

