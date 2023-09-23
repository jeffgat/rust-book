// In Function Definitions

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

// In Struct Definitions
struct Point<T> {
    x: T,
    y: T,
}
struct PointAlt<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 60, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number is {}", result);

    let char_list = vec!['y', 'b', 'c', 'q'];
    let result = largest(&char_list);
    println!("largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.2, y: 10.3 };
    let point_alt = PointAlt { x: 5, y: 10.3 };

    println!("main",);
}
