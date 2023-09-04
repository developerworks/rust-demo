#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct User<'a> {
    name: &'a str,
    email: &'a str,
}

fn main() {
    let x = Point { x: 1, y: 2 };
    let y = x;
    println!("{:?}", x);
    println!("{:?}", y);

    let u = User {
        name: "John Doe",
        email: "john@example.com",
    };

    // let user = u;

    // println!(" user name: {}", user.name);
    // println!("user email: {}", user.email);

    println!("Address of          u: {:p}", std::ptr::addr_of!(u));
    // println!("Address of       user: {:p}", std::ptr::addr_of!(user));
    println!("Address of     u.name: {:p}", std::ptr::addr_of!(u.name));
    println!("Address of    u.email: {:p}", std::ptr::addr_of!(u.email));
    // println!("Address of  user.name: {:p}", std::ptr::addr_of!(user.name));
    // println!("Address of user.email: {:p}", std::ptr::addr_of!(user.email));
}
