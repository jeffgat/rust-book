fn main() {
    let x = 5;

    let x = x + 1;

    let z = 'c';
    let b = "string literal uses double quotes";

    let tup: (i32, f64, u8) = (499, 6.3, 2);

    let (r, y, z) = tup;

    println!("The value of y is: {y}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
}