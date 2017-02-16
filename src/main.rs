use std::collections::HashMap;

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 3];
    let mut total = 0;
    let length = nums.len();
    let mut map = HashMap::new();

    for i in &nums {
        total += *i;
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    println!("Mean: {:.2}", f64::from(total) * 1. / f64::from(length as i32));

    nums.sort();

    let mode_index = if length % 2 == 0 { length / 2 } else { (length - 1) / 2 };

    println!("Median: {}", nums[mode_index]);

    let mut freq = (0, 0);
    for (entry, value) in map.into_iter() {
        if freq.1 < value {
            freq = (entry, value);
        }
    }

    println!("Mode: {}", freq.0);
}
