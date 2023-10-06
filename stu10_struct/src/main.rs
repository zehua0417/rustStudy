/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion study struct
 */
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let user1 = User{
        email: String::from("zehuali0417@gmail.com"),
        username: String::from("Lihuax"),
        active: true,
        sign_in_count: 556,
    };
    let user2 = User{
        email: String::from("lizehua21@lzu.edu.cn"),
        username: String::from("Lizehua"),
        ..user1
    };
    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    

}