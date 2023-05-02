fn main() {
    println!("Hello, world!");

    fn fahrenheit_to_celsius(temp_in_f: f32) -> f32 {
        (temp_in_f - 32.) / 1.8
    }

    let c = fahrenheit_to_celsius(68.);
    println!("celsius is {c}");

    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
    fn fib(num: i32) -> i32 {
        let mut prev = 0;
        let mut curr = 1;

        // for _ in 0..num {
        //     let next = prev + curr;
        //     prev = curr;
        //     curr = next;
        // }

        let mut count = 0;
        // loop {
        //     if count == num - 1 {
        //         break;
        //     }
        //     let next = prev + curr;
        //     prev = curr;
        //     curr = next;
        //     count += 1
        // }

        while count < num - 1 {
            let next = prev + curr;
            prev = curr;
            curr = next;
            count += 1
        }
        prev
    }

    let fib_num = fib(6);
    println!("fib sequence number is {fib_num}");
}
