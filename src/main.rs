#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let rectangle = Rectangle { 
        width: 31,
         height: 30
    };
    let area = calculate_area(&rectangle);

    println!("The area of {:#?} is: {}", rectangle, area);
}

fn calculate_area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

/* 
struct Color (u8, u8, u8);
struct Point (u8, u8, u8);

fn main() {

    let red = Color(100, 0, 0);
    set_color(red);

    let point = Point(30, 40, 20);
    move_point(point);
}

fn set_color(color: Color) {
    println!("Setting background color R = {}, G = {}, B = {}", 
    color.0, color.1, color.2
    )
}

fn move_point(point: Point) {
    println!(
        "The cursor was moved X = {}, Y = {}, Z = {}" ,
        point.0, point.1, point.2
    )
} */

/* 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = build_user(
        String::from("Sheryar"),
        String::from("Sheryar@gmail.com"), 
    );

    let user2 = User {
        email: String::from("Tahir@gmail.com"),
        ..user1
    };

    println!("Email of user is: {}", user2.email);
}

fn build_user(username: String, email: String) -> User {
    User { 
        active: true,
        username,
        email,
        sign_in_count: 0 
    }
} */