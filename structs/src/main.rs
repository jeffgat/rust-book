fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };

    let mut user1 = User {
        active: true,
        username: String::from("dude1"),
        email: String::from("dude@dude.com"),
        sign_in_count: 4,
    };

    user1.email = String::from("newdude@dude.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 2,
        }
    }

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // becomes
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // however this `moves` the data and we can no longer access user1

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {:#?}", rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
