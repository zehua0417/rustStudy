/**
 * @file main.rs
 * @author lihuax
 * @version 0.1
 * @descritpion
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main(){
    //let width1 = 30;
    //let height1 = 50;
    //let rect1 = (30, 50);
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("{:#?}", rect1);

    println!("{}", area(&rect1));
}