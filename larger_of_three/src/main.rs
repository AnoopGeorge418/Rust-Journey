use std::io::stdin;

fn main() {
    println!("Largest Of 3 numbers!");

    let mut input = String::new();
    println!("Enter 3 numbers separated by spaces:");
    stdin().read_line(&mut input).expect("Failed to read user input");

    // Parse the space-separated input into a vector of i32
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    assert_eq!(numbers.len(), 3, "Please enter exactly 3 numbers");
    // let nums_array: [i32; 3] = [numbers[0], numbers[1], numbers[2]];
    let nums_array: [i32; 3] = numbers.try_into().expect("Expected exactly 3 numbers");
    
    let largest: i32 = largest_of_all(nums_array);

    println!("Largest number in {nums_array:?} is: {largest} ");
}

fn largest_of_all(nums: [i32; 3]) -> i32 {
    let mut largest_num: i32 = nums[0];
    
    for number in nums {
        if number > largest_num {
            largest_num = number;
        }
    }

    largest_num
}
