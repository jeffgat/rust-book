// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // so x and y have to be the same time
    struct PointSingle<T> {
        x: T,
        y: T,
    }

    // whereas x and y can be of type t or u
    struct PointDouble<T, U> {
        x: T,
        y: U,
    }

    struct PointX<T> {
        x: T,
        y: T,
    }

    impl<T> PointX<T> {
        // this function is basically like this.x, it just returns x from the struct above
        fn x(&self) -> &T {
            &self.x
        }
    }
}
