fn main() {
    println!("Hello, world!");

    fn fahrenheit_to_celsius(temp_in_f: f32) -> f32 {
        (temp_in_f - 32.) / 1.8
    }

    let c = fahrenheit_to_celsius(68.);
    println!("celsius is {c}");

    fn fib(num: i32) -> {
        // todo
    }

    let fibNum = fib(6)
    println!("fib sequence number is {fibNum}")
}