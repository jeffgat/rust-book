use std::collections::HashMap;
use std::io;

fn main() {
    let list = vec![1, 3, 3, 4, 2];

    fn get_median(list: &Vec<i32>) -> f32 {
        let mut sorted_list = list.clone();
        sorted_list.sort();
        let len = sorted_list.len();
        // as these are ints, we need to cast to f32 to get a float
        // if we wanted that behaviour
        // since we just want the whole number,
        // this gives us the middle index
        let mid = len / 2;
        println!("len: {}", len);
        println!("mid: {}", mid);

        if len % 2 == 0 {
            (sorted_list[mid] + sorted_list[mid - 1]) as f32 / 2.0
        } else {
            sorted_list[mid] as f32
        }
    }

    fn get_mode(list: &Vec<i32>) -> i32 {
        let mut mode = 0;
        let mut highest_count = 0;
        let mut counts = HashMap::new();

        for item in list {
            let count = counts.entry(item).or_insert(0);
            *count += 1;
            if *count > highest_count {
                highest_count = *count;
                mode = *item;
            }
        }
        println!("counts: {:?}", counts);
        mode
    }

    println!("{}", get_median(&list));
    println!("{}", get_mode(&list));

    let word = "hello";

    fn convert_to_pig_latin(word: &str) -> String {
        // .chars() is akin to .split('') in js
        let mut chars = word.chars();
        // .next() continues to iterate
        // if we need to go deep we can use .nth()
        // .unwrap() unwraps the value from the Option
        // which is Some or None
        let first_char = chars.next().unwrap();
        let mut new_word = String::new();
        new_word.push_str(&word[1..]);
        new_word.push(first_char);
        new_word.push_str("ay");
        new_word
    }
    println!("{}", convert_to_pig_latin(word));
}
