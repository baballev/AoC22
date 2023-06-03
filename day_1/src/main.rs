use std::fs;

/**
 * The problem is a simple one dimensional optimization problem (maximum search).
 * It's a good opportunity to familiarize with the I/O system of Rust, and more specifically,
 * get used to reading the standard input. For the 1st excercise, the input will be read from
 * the file ./input.txt while for the second we will read the input directly in stdin.
 */
fn main() {
    first_excercise();
    second_excercise();
}

const INPUT_PATH: &str = "./input.txt";

fn first_excercise() {
    let input_content = fs::read_to_string(INPUT_PATH).expect(&format!(
        "An error occured while reading the input file at path: {}",
        INPUT_PATH
    ));
    let mut max_calories_carried = 0;
    let mut running_calories = 0;
    let input_lines = input_content.lines();
    for line in input_lines {
        if line.len() == 0 {
            compare_and_keep_max(&mut max_calories_carried, &mut running_calories, 0)
        } else {
            running_calories += line.parse::<i32>().unwrap(); // TODO: Cast line into a number
        }
    }
    println!("{}", max_calories_carried)
}

fn compare_and_keep_max<T: std::cmp::PartialOrd + Copy>(
    max_calories_carried: &mut T,
    running_calories: &mut T,
    reset_value: T,
) {
    if *running_calories > *max_calories_carried {
        *max_calories_carried = *running_calories;
        *running_calories = reset_value;
    }
}

fn second_excercise() {
    let input_content = fs::read_to_string(INPUT_PATH).unwrap();
    let mut top_3_calories: Vec<u32> = vec![0, 0, 0];
    let mut current_elf_calories: u32 = 0;
    for line in input_content.lines() {
        if line.len() == 0 {
            for (index, top_calorie) in top_3_calories.iter().enumerate() {
                if current_elf_calories > *top_calorie {
                    push_value_from(current_elf_calories, index, &mut top_3_calories);
                    break;
                }
            }
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<u32>().unwrap();
        }
    }
    println!("Let's find out the results of the calorie inspection!");
    let mut sum_top_3_calories: u32 = 0;
    for top_calorie in top_3_calories.iter() {
        println!("{}", top_calorie);
        sum_top_3_calories += *top_calorie;
    }
    println!("Sum of top 3 calories: {}", sum_top_3_calories)
}

fn push_value_from(value_to_push: u32, index_from: usize, vector: &mut Vec<u32>) {
    let mut previous_value;
    let mut next_value = value_to_push;
    for index in index_from..vector.len() {
        previous_value = vector[index];
        vector[index] = next_value;
        next_value = previous_value;
    }
}
