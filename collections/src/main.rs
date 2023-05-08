use std::collections::HashMap;

fn main() {
    // because we initialize v as empty, we need to declare the type explicity
    // let v: Vec<i32> = Vec::new();

    // vec! macro will create a vector from the values given -> ccan infer the types in this case cause it's not empty
    // let v2 = vec![1, 2, 3];

    // let mut v3 = Vec::new();

    // v3.push(5);
    // v3.push(6);
    // v3.push(7);
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    println!("v4: {:?}", v4);

    let x = 5;
    let y = &x;

    println!("{}", x);

    // concatting with + operator
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // concatting with format! macro
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");

    // hash maps

    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("{}", score)

    // entry/or_insert method
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);
}
