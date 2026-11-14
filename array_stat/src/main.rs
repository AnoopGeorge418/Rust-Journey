use std::io::stdin;

fn main() {
    println!("Array Statistics!");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("No numbers entered!");
        return;
    }

    if numbers.len() <= 1 {
        println!("Enter more than 1 number to perform calculations!");
        return;
    }

    let sum: i32 = sum_of_array(&numbers);
    let avg: i32 = average_of_array(&numbers);
    let min: i32 = min_of_array(&numbers);
    let max: i32 = max_of_array(&numbers);
    let even: i32 = even_count_of_array(&numbers);
    let odd: i32 = odd_count_of_array(&numbers);

    println!("Sum: {}", sum);
    println!("Average: {}", avg);
    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Even count: {}", even);
    println!("Odd count: {}", odd);
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut sorted_array: Vec<i32> = arr.to_vec();
    let len = sorted_array.len();

    // bubble sort
    for i in 0..len {
        for j in 0..len - 1 - i {
            if sorted_array[j] > sorted_array[j + 1] {
                let temp = sorted_array[j];
                sorted_array[j] = sorted_array[j + 1];
                sorted_array[j + 1] = temp;
            }
        }
    }

    sorted_array
}

fn sum_of_array(arr: &[i32]) -> i32 {
    let sorted_arr = sort_array(arr);

    let mut sum = 0;
    for num in sorted_arr {
        sum += num;
    }

    sum
}

fn average_of_array(arr: &[i32]) -> i32 {
    let len: i32 = arr.len() as i32;
    let sum: i32 = sum_of_array(arr);

    sum / len
}

fn min_of_array(arr: &[i32]) -> i32 {
    let sorted_arr = sort_array(arr);
    sorted_arr[0]
}

fn max_of_array(arr: &[i32]) -> i32 {
    let sorted_arr = sort_array(arr);
    let len = sorted_arr.len();

    sorted_arr[len - 1]
}

fn even_count_of_array(arr: &[i32]) -> i32 {
    let sorted_arr = sort_array(arr);
    let mut count = 0;

    for element in sorted_arr {
        if element % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn odd_count_of_array(arr: &[i32]) -> i32 {
    let sorted_arr = sort_array(arr);
    let mut count = 0;

    for element in sorted_arr {
        if element % 3 != 0 {
            count += 1;
        }
    }

    count
}
