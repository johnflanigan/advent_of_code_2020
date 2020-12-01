use std::collections::HashSet;
use std::env;
use std::fs;

pub fn run() {
    let mut path = env::current_dir().expect("Error getting current directory");
    path.push("src");
    path.push("input");
    path.push("day1_input.txt");
    let filename = path.to_str().expect("Error getting path as string");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let nums: HashSet<i64> = contents
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    println!("Part one solution: {}", two_sum(nums.clone(), 2020));
    println!("Part two solution: {}", three_sum(nums, 2020));
}

fn two_sum(nums: HashSet<i64>, sum: i64) -> i64 {
    let mut set: HashSet<i64> = HashSet::new();

    for num in nums {
        if set.contains(&(sum - num)) {
            return (sum - num) * num;
        } else {
            set.insert(num);
        }
    }

    return -1;
}

fn three_sum(nums: HashSet<i64>, sum: i64) -> i64 {
    for num in nums.clone() {
        let mut temp = nums.clone();
        temp.remove(&num);

        let solution = two_sum(temp, sum - num);
        if solution != -1 {
            return num * solution;
        }
    }

    return -1;
}
